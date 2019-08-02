pub mod mwl_ad;

#[derive (Debug, PartialEq)]
pub struct DiodePos<P,T>
{
    pub diode_front: (P,P),
    pub diode_back: (P,P),
    pub time: T
}

#[derive (Debug, PartialEq)]
pub enum DiodeId
{
    Front,
    Back,
}

#[derive (Debug, PartialEq)]
pub struct DiodeImage<P,T>
{
    pub diode_id: DiodeId,
    pub coords: Vec<(P,P)>,
    pub time: T,
}
