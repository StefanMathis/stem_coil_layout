use stem_coil_layout::*;

#[test]
fn test_zone_ordering() {
    {
        let zone_1 = Zone::new(0, 0);
        let zone_2 = Zone::new(1, 0);
        assert!(zone_1 < zone_2)
    }
    {
        let zone_1 = Zone::new(0, 0);
        let zone_2 = Zone::new(0, 1);
        assert!(zone_1 < zone_2)
    }
    {
        let zone_1 = Zone::new(0, 1);
        let zone_2 = Zone::new(0, 1);
        assert!(zone_1 == zone_2)
    }
    {
        let zone_1 = Zone::new(2, 0);
        let zone_2 = Zone::new(0, 1);
        assert!(zone_1 > zone_2)
    }
}
