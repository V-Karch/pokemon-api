pub struct Move {
    pub id: i32,
    pub name: String,
    pub move_type: String,
    pub category: String,
    pub power: Option<i32>,
    pub accuracy: Option<i32>,
    pub pp: Option<i32>,
    pub effect: Option<String>,
    pub probability: Option<i32>,
}

impl Move {
    pub fn to_string(&self) -> String {
        return format!(
            "Move {{ id: {}, name: \"{}\", move_type: \"{}\", category: \"{}\", power: {}, accuracy: {}, pp: {}, effect: \"{}\", probability: {}}}",
            &self.id,
            &self.name,
            &self.move_type,
            &self.category,
            match &self.power {
                Some(value) => value.to_string(),
                None => "null".to_string(),
            },
            match &self.accuracy {
                Some(value) => value.to_string(), 
                None => "null".to_string(),
            },
            match &self.pp {
                Some(value) => value.to_string(),
                None => "null".to_string(),
            },
            match &self.effect {
                Some(value) => value.to_string(),
                None => "null".to_string(),
            },
            match &self.probability {
                Some(value) => value.to_string(),
                None => "null".to_string(),
            }
        );
    }
}