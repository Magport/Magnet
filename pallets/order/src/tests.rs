use crate::mock::*;

#[test]
fn order_default_value() {
	ExtBuilder::default().build().execute_with(|| {
		assert_eq!(OrderPallet::slot_width(), 2);
	});
}
