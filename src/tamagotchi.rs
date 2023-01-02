/// This is the container for the [Tamagotchi].
///
/// This is where the user will interact with their 'gotchi.
pub(crate) struct TamagotchiInterface {
    display: InterfaceDisplay,
    actions: Actions,
    tamagotchi: Tamagotchi,
}
/// These are the possible screen displays the user can navigate to.
pub(crate) enum InterfaceDisplay {
    Clock,
    HungerMeter,
    HappinessMeter,
    Character,
    DisciplineMeter,
    AgeAndWeight,
    ReturningHome,
}
/// These are the possible actions the user can take to interact with their 'gotchi.
pub(crate) enum Actions {
    Feed,
    Light,
    Play,
    Medicine,
    Duck,
    HealthMeter,
    Attention,
    Discipline,
}
/// Translates to "Egg-watch".
///
/// This is the actual character the user will interact with.
pub(crate) struct Tamagotchi {
    name: String,
    gender: Gender,
    age: u32,
    weight: f64,
    form: Form,
    status: Status,
}
impl Tamagotchi {
    /// feed your tamagotchi
    pub(crate) fn feed(&self) -> Self {
        let status = self.status.eat();
        Self {
            name: self.name.clone(),
            gender: self.gender.clone(),
            weight: self.weight + 0.5,
            form: self.form.clone(),
            status,
            ..*self
        }
    }
    /// turn the light on or off
    pub(crate) fn light(&self) -> Self {
        let status = match self.status.light {
            Light::On => Light::Off,
            Light::Off => Light::On,
        };
        match self.status.asleep {
            true => {}
            false => {}
        }
        Self { ..*self }
    }
    /// play with your tamagotchi
    pub(crate) fn play(&self) -> Self {
        let status = self.status.play();
        Self {
            name: self.name.clone(),
            gender: self.gender.clone(),
            weight: self.weight - 0.2,
            form: self.form.clone(),
            status,
            ..*self
        }
    }
    /// give your tamagotchi medicine if it's sick
    pub(crate) fn give_medicine(&self) -> Self {
        Self { ..*self }
    }
    /// clean up after your tamagotchi
    pub(crate) fn duck(&self) -> Self {
        Self { ..*self }
    }
    /// give your tamagotchi attention
    pub(crate) fn attention(&self) -> Self {
        Self { ..*self }
    }
    /// discipline your tamagotchi if they are bad
    pub(crate) fn discipline(&self) -> Self {
        self
    }
}
/// The gender of a [Tamagotchi].
#[derive(Clone)]
pub(crate) enum Gender {
    Male,
    Female,
}
/// The stage of evolution of a [Tamagotchi]'s life.
#[derive(Clone)]
pub(crate) enum Form {
    /// Literally translates to "Egg".
    Tamago,
    /// Literally translates to "white baby-watch", hatches from egg after 5 mins.
    /// Base weight is 5lbs.
    Shirobabytchi,
    /// The "child" stage which evolves after 65 mins.
    /// Base weight is 10lbs.
    Tonmarutchi,
    /// Teen form depends on [CareLevel].
    Teen(TeenForm),
    Adult(AdultForm),
    Special(SpecialForm),
}
impl Default for Form {
    fn default() -> Self {
        Form::Tamago
    }
}
/// Possible teenager [Tamagotchi] forms.
#[derive(Clone)]
pub(crate) enum TeenForm {
    /// Good care, 75% [DisciplineLevel]
    Tongaritchi,
    /// Bad care, 75% [DisciplineLevel]
    Hashitamatchi,
}
/// Possible adult [Tamagotchi] forms.
#[derive(Clone)]
pub(crate) enum AdultForm {
    Mimitchi,
    Pochitchi,
    Nyatchi,
    Zuccitchi,
    Hashizoutchi,
    Kusatchi,
    Takotchi,
}
/// Some adult 'gotchis can evolve past [AdultForm] and become _special_.
/// These are the possible special [Tamagotchi] forms.
#[derive(Clone)]
pub(crate) enum SpecialForm {
    Sekitoritchi,
    Charitchi,
    Zatchi,
}
/// Aids in defining a point system for the number of hearts in a heart gauge.
///
/// Example: ♥️ ♡ ♡ ♡
trait Hearts {
    fn hearts(&self) -> u32;
}
/// For use in incrementing or decrementing the [Status] conditions
/// of a [Tamagotchi].
trait BetterOrWorse {
    fn better(&self) -> Self;
    fn worse(&self) -> Self;
}
/// The condition of a [Tamagotchi].
pub(crate) struct Status {
    care: CareLevel,
    hunger: Hunger,
    light: Light,
    asleep: bool,
    mood: Mood,
    sick: bool,
    soiled: bool,
    health: Health,
    discipline: Discipline,
}
impl Status {
    /// Determines the user's level of care toward a [Tamagotchi].
    pub(crate) fn care_level(&self) -> CareLevel {
        let care = self.hunger.hearts()
            + self.mood.hearts()
            + self.health.hearts()
            + self.discipline.meter();
        match care {
            0 | 1 | 2 | 3 => CareLevel::Bad,
            4 | 5 | 6 | 7 => CareLevel::BelowAverage,
            8 | 9 | 10 | 11 => CareLevel::AboveAverage,
            12 | 13 | 14 | 15 => CareLevel::Good,
            16 => CareLevel::Perfect,
            _ => unreachable!("Maximum care level is 16"),
        }
    }
    /// [Status] changes for when a [Tamagotchi] eats.
    pub(crate) fn eat(&self) -> Status {
        let hunger = self.hunger.better();
        Self { hunger, ..*self }
    }
    /// [Status] changes for when a [Tamagotchi] plays.
    pub(crate) fn play(&self) -> Status {
        let mood = self.mood.better();
        Self { mood, ..*self }
    }
    /// [Status] changes for when a [Tamagotchi] sleeps.
    pub(crate) fn sleep(&self) -> Status {
        let mood = self.mood.better();
        let health = self.health.better();
        Self {
            mood,
            health,
            ..*self
        }
    }
}
/// The level of care a [Tamagotchi] receives.
pub(crate) enum CareLevel {
    Bad,
    BelowAverage,
    AboveAverage,
    Good,
    Perfect,
}
impl Default for CareLevel {
    fn default() -> Self {
        CareLevel::Good
    }
}
/// The hunger level of a [Tamagotchi].
pub(crate) enum Hunger {
    Starving,
    Famished,
    Snackish,
    Peckish,
    Full,
}
impl Default for Hunger {
    fn default() -> Self {
        Hunger::Starving
    }
}
impl Hearts for Hunger {
    fn hearts(&self) -> u32 {
        match self {
            Self::Starving => 0,
            Self::Famished => 1,
            Self::Snackish => 2,
            Self::Peckish => 3,
            Self::Full => 4,
        }
    }
}
impl BetterOrWorse for Hunger {
    fn better(&self) -> Self {
        match self {
            Hunger::Starving => Hunger::Famished,
            Hunger::Famished => Hunger::Snackish,
            Hunger::Snackish => Hunger::Peckish,
            Hunger::Peckish => Hunger::Full,
            Hunger::Full => Hunger::Full,
        }
    }
    fn worse(&self) -> Self {
        match self {
            Hunger::Starving => Hunger::Starving,
            Hunger::Famished => Hunger::Starving,
            Hunger::Snackish => Hunger::Famished,
            Hunger::Peckish => Hunger::Snackish,
            Hunger::Full => Hunger::Peckish,
        }
    }
}
/// The [Tamagotchi]'s room light.
pub(crate) enum Light {
    On,
    Off,
}
/// The mood of a [Tamagotchi].
pub(crate) enum Mood {
    Miserable,
    Pessemistic,
    Indifferent,
    Optimistic,
    Cheerful,
}
impl Default for Mood {
    fn default() -> Self {
        Mood::Miserable
    }
}
impl Hearts for Mood {
    fn hearts(&self) -> u32 {
        match self {
            Mood::Miserable => 0,
            Mood::Pessemistic => 1,
            Mood::Indifferent => 2,
            Mood::Optimistic => 3,
            Mood::Cheerful => 4,
        }
    }
}
impl BetterOrWorse for Mood {
    fn better(&self) -> Self {
        match self {
            Mood::Miserable => Mood::Pessemistic,
            Mood::Pessemistic => Mood::Indifferent,
            Mood::Indifferent => Mood::Optimistic,
            Mood::Optimistic => Mood::Cheerful,
            Mood::Cheerful => Mood::Cheerful,
        }
    }
    fn worse(&self) -> Self {
        match self {
            Mood::Miserable => Mood::Miserable,
            Mood::Pessemistic => Mood::Miserable,
            Mood::Indifferent => Mood::Pessemistic,
            Mood::Optimistic => Mood::Indifferent,
            Mood::Cheerful => Mood::Optimistic,
        }
    }
}
/// A [Tamagotchi]'s health condition.
pub(crate) enum Health {
    Neglected,
    Weak,
    Normal,
    Strong,
    Eggcellent,
}
impl Default for Health {
    fn default() -> Self {
        Health::Neglected
    }
}
impl Hearts for Health {
    fn hearts(&self) -> u32 {
        match self {
            Health::Neglected => 0,
            Health::Weak => 1,
            Health::Normal => 2,
            Health::Strong => 3,
            Health::Eggcellent => 4,
        }
    }
}
impl BetterOrWorse for Health {
    fn better(&self) -> Self {
        match self {
            Health::Neglected => Health::Weak,
            Health::Weak => Health::Normal,
            Health::Normal => Health::Strong,
            Health::Strong => Health::Eggcellent,
            Health::Eggcellent => Health::Eggcellent,
        }
    }
    fn worse(&self) -> Self {
        match self {
            Health::Neglected => Health::Neglected,
            Health::Weak => Health::Neglected,
            Health::Normal => Health::Weak,
            Health::Strong => Health::Normal,
            Health::Eggcellent => Health::Strong,
        }
    }
}
/// The behavior of a [Tamagotchi].
///
/// This may get refactored.
pub(crate) enum Behavior {
    Good,
    Bad,
}
impl Behavior {
    pub(crate) fn reduce() -> Behavior {
        match rand::random() {
            true => Behavior::Good,
            false => Behavior::Bad,
        }
    }
}
/// The level of discipline a [Tamagotchi] displays.
pub(crate) enum Discipline {
    Bratty,
    Spoiled,
    Average,
    Goody2Shoes,
    ModelAlien,
}
impl Discipline {
    pub(crate) fn meter(&self) -> u32 {
        match self {
            Discipline::Bratty => 0,
            Discipline::Spoiled => 1,
            Discipline::Average => 2,
            Discipline::Goody2Shoes => 3,
            Discipline::ModelAlien => 4,
        }
    }
}
