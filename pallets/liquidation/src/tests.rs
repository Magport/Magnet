use super::*;
use crate::mock::*;
use frame_support::assert_ok;
use sp_runtime::traits::Zero;

#[test]
fn distribute_profit_should_work() {
	ExtBuilder::default()
		.existential_deposit(100)
		.system_ratio(20_000_0000)
		.treasury_ratio(33_000_0000)
		.operation_ratio(vec![(AccountId32::new([1u8; 32]), 25_000_0000)])
		.collator_ratio(22_000_0000)
		.min_liquidation_threshold(20_000_000_000)
		.profit_distribution_cycle(10)
		.build()
		.execute_with(|| {
			for n in 1..=ProfitDistributionCycle::<Test>::get() {
				frame_system::Pallet::<Test>::set_block_number(n.into());
			}

			assert_ok!(Liquidation::test_distribute_profit());
			let events = frame_system::Pallet::<Test>::events()
				.into_iter()
				.map(|record| record.event)
				.filter_map(|event| {
					if let RuntimeEvent::Liquidation(inner_event) = event {
						Some(inner_event)
					} else {
						None
					}
				})
				.collect::<Vec<_>>();

			assert!(events.iter().any(expected_event));
			assert_eq!(<TotalIncome<Test>>::get(), Zero::zero());
			assert_eq!(<TotalCost<Test>>::get(), Zero::zero());
			assert!(<CollatorRealGasCosts<Test>>::iter().next().is_none());
		});
}

#[test]
fn set_operation_ratios_should_work() {
	ExtBuilder::default().build().execute_with(|| {
		let new_ratios = vec![
			(AccountId32::new([3u8; 32]), 30_000_0000),
			(AccountId32::new([4u8; 32]), 10_000_0000),
		];
		assert_ok!(Liquidation::set_operation_ratios(RuntimeOrigin::root(), new_ratios.clone()));
		for (account, ratio) in new_ratios {
			assert_eq!(OperationRatios::<Test>::get(account), ratio);
		}

		let total_ratio = SystemRatio::<Test>::get()
			+ TreasuryRatio::<Test>::get()
			+ CollatorRatio::<Test>::get()
			+ OperationRatios::<Test>::iter().map(|(_, r)| r).sum::<u32>();
		assert!(total_ratio <= 100 * PERCENT_UNIT as u32);
	});
}
