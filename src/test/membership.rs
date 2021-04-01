use crate::models::membership::MembershipType;

#[test]
fn membership_serialize() {
    assert_eq!(MembershipType::None.into_i32(), 0);
    assert_eq!(MembershipType::Xbox.into_i32(), 1);
    assert_eq!(MembershipType::Psn.into_i32(), 2);
    assert_eq!(MembershipType::Steam.into_i32(), 3);
    assert_eq!(MembershipType::Blizzard.into_i32(), 4);
    assert_eq!(MembershipType::BungieNet.into_i32(), 254);
    assert_eq!(MembershipType::All.into_i32(), -1);
}