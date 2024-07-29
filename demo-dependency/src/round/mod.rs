#[near_sdk::near(serializers = [borsh, json])]
pub enum RoundStatus {
    Running,
    Pending,
}
