
#[derive(Serialize, Deserialize)]
pub struct Output {
    pub message: String,
    pub status: String,
    pub data: DbData,
}

#[derive(Serialize, Deserialize)]
pub struct DbData {
    pub timestamp: String,
    pub sensor: String,
    pub measurement: f32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AlarmInformation {
    pub name:String,
    pub status:bool,
    pub trigger:f32,
    pub fire_message:String,
    pub normal_message:String,
}
