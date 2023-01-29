use serde::{Serialize, Deserialize};

// https://github.com/goecharger/go-eCharger-API-v1/blob/master/go-eCharger%20API%20v1%20EN.md
#[derive(Serialize, Deserialize)]
pub(crate) struct Status {
    version: String,
    /// Status PWM Signaling
    /// 1: charging station ready, no vehicle
    /// 2: vehicle loads
    /// 3: Waiting for vehicle
    /// 4: Charge finished, vehicle still connected
    car: String,
    
    /// Ampere value for the PWM signaling in whole ampere of 6-32A
    /// Will not be written on flash but acts like you set amp instead. 
    /// Only on the next reboot, the amp value will be restored to the 
    /// last value set with amp. 
    /// Recommended for PV charging
    amt: String,
    
    /// Charged energy in deca-watt seconds
    /// Example: 100'000 means, 1'000'000 Ws (= 277Wh = 0.277kWh)
    /// were charged during this charging process.
    dws: String,

    /// energy_total: Total charged energy in 0.1kWh
    /// Example: 130 means 13kWh charged
    eto: String,
}
impl Status {
    pub(crate) fn energy(&self) -> i64 {
        self.eto.parse::<i64>().expect("eto is a number")
    }
}


pub(crate) async fn status() ->  Result<Status, reqwest::Error>  {
    let status: Status = reqwest::get("http://192.168.178.53/status")
        .await?
        .json()
        .await?;

    Ok(status)
}
