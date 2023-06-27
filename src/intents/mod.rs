#[derive(Clone, Debug)]
pub struct Intents {
    pub intents: u32,
}
#[allow(non_camel_case_types)]
pub enum IntentsBits {
    ALL,
    GUILDS,
    GUILD_MEMBERS,
    GUILD_BANS,
    GUILD_EMOJIS_AND_STICKERS,
    GUILD_INTEGRATIONS,
    GUILD_WEBHOOKS,
    GUILD_INVITES,
    GUILD_VOICE_STATES,
    GUILD_PRESENCES,
    GUILD_MESSAGES,
    GUILD_MESSAGE_REACTIONS,
    GUILD_MESSAGE_TYPING,
    DIRECT_MESSAGES,
    DIRECT_MESSAGE_REACTIONS,
    DIRECT_MESSAGE_TYPING,
    MESSAGE_CONTENT,
    GUILD_SCHEDULED_EVENTS,
    AUTO_MODERATION_CONFIGURATION,
    AUTO_MODERATION_EXECUTION,
}

impl Intents {
    pub fn new() -> Intents {
        Intents { intents: 0 }
    }
    pub fn builder() -> IntentsBuilder {
        IntentsBuilder {
            intents: 0
        }
    }
}
pub struct IntentsBuilder {
    intents: u32,
}
impl IntentsBuilder {
    pub fn set_intent(mut self, intent_bit: u32) -> IntentsBuilder {
        self.intents = intent_bit;
        self
    }
    pub fn add_intent(mut self, intent: IntentsBits) -> IntentsBuilder {
        match intent {
            IntentsBits::GUILDS => {
                self.intents += 1 << 0;
                self
            }
            IntentsBits::GUILD_MEMBERS => {
                self.intents += 1 << 1;
                self
            }
            IntentsBits::GUILD_BANS => {
                self.intents += 1 << 2;
                self
            }
            IntentsBits::GUILD_EMOJIS_AND_STICKERS => {
                self.intents += 1 << 3;
                self
            }
            IntentsBits::GUILD_INTEGRATIONS => {
                self.intents += 1 << 4;
                self
            }
            IntentsBits::GUILD_WEBHOOKS => {
                self.intents += 1 << 5;
                self
            }
            IntentsBits::GUILD_INVITES => {
                self.intents += 1 << 6;
                self
            }
            IntentsBits::GUILD_VOICE_STATES => {
                self.intents += 1 << 7;
                self
            }
            IntentsBits::GUILD_PRESENCES => {
                self.intents += 1 << 8;
                self
            }
            IntentsBits::GUILD_MESSAGES => {
                self.intents += 1 << 9;
                self
            }
            IntentsBits::GUILD_MESSAGE_REACTIONS => {
                self.intents += 1 << 10;
                self
            }
            IntentsBits::GUILD_MESSAGE_TYPING => {
                self.intents += 1 << 11;
                self
            }
            IntentsBits::DIRECT_MESSAGES => {
                self.intents += 1 << 12;
                self
            }
            IntentsBits::DIRECT_MESSAGE_REACTIONS => {
                self.intents += 1 << 13;
                self
            }
            IntentsBits::DIRECT_MESSAGE_TYPING => {
                self.intents += 1 << 14;
                self
            }
            IntentsBits::MESSAGE_CONTENT => {
                self.intents += 1 << 15;
                self
            }
            IntentsBits::GUILD_SCHEDULED_EVENTS => {
                self.intents += 1 << 16;
                self
            }
            IntentsBits::AUTO_MODERATION_CONFIGURATION => {
                self.intents += 1 << 20;
                self
            }
            IntentsBits::AUTO_MODERATION_EXECUTION => {
                self.intents += 1 << 21;
                self
            }
            IntentsBits::ALL => {
                self.intents = 3276799;
                self
            }
            _ => self,
        }
    }
    pub fn add_intents(mut self, intents: Vec<IntentsBits>) -> IntentsBuilder {
        let _ = intents.iter().map(|intent| match intent {
            IntentsBits::GUILDS => {
                self.intents += 1 << 0;
            }
            IntentsBits::GUILD_MEMBERS => {
                self.intents += 1 << 1;
            }
            IntentsBits::GUILD_BANS => {
                self.intents += 1 << 2;
            }
            IntentsBits::GUILD_EMOJIS_AND_STICKERS => {
                self.intents += 1 << 3;
            }
            IntentsBits::GUILD_INTEGRATIONS => {
                self.intents += 1 << 4;
            }
            IntentsBits::GUILD_WEBHOOKS => {
                self.intents += 1 << 5;
            }
            IntentsBits::GUILD_INVITES => {
                self.intents += 1 << 6;
            }
            IntentsBits::GUILD_VOICE_STATES => {
                self.intents += 1 << 7;
            }
            IntentsBits::GUILD_PRESENCES => {
                self.intents += 1 << 8;
            }
            IntentsBits::GUILD_MESSAGES => {
                self.intents += 1 << 9;
            }
            IntentsBits::GUILD_MESSAGE_REACTIONS => {
                self.intents += 1 << 10;
            }
            IntentsBits::GUILD_MESSAGE_TYPING => {
                self.intents += 1 << 11;
            }
            IntentsBits::DIRECT_MESSAGES => {
                self.intents += 1 << 12;
            }
            IntentsBits::DIRECT_MESSAGE_REACTIONS => {
                self.intents += 1 << 13;
            }
            IntentsBits::DIRECT_MESSAGE_TYPING => {
                self.intents += 1 << 14;
            }
            IntentsBits::MESSAGE_CONTENT => {
                self.intents += 1 << 15;
            }
            IntentsBits::GUILD_SCHEDULED_EVENTS => {
                self.intents += 1 << 16;
            }
            IntentsBits::AUTO_MODERATION_CONFIGURATION => {
                self.intents += 1 << 20;
            }
            IntentsBits::AUTO_MODERATION_EXECUTION => {
                self.intents += 1 << 21;
            }
            IntentsBits::ALL => {
                self.intents = 3276799;
            }
        });
        self
    }
    pub fn build(self) -> Intents {
        Intents {
            intents: self.intents,
        }
    }
}
