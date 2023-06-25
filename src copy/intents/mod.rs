pub struct Intents {
    intents: u64,
}
pub struct IntentsBuilder {
    intents: u64,
}
impl Intents {
    pub fn builder() -> IntentsBuilder {
        IntentsBuilder { intents: 0 }
    }
}
impl IntentsBuilder {
    pub fn GUILDS(mut self) -> IntentsBuilder {
        self.intents = 1;
        self
    }
    pub fn GUILD_MEMBERS(mut self) -> IntentsBuilder {
        self.intents = 1 << 1;
        self
    }
    pub fn GUILD_BANS(mut self) -> IntentsBuilder {
        self.intents = 1 << 2;
        self
    }
    pub fn GUILD_EMOJIS_AND_STICKERS(mut self) -> IntentsBuilder {
        self.intents = 1 << 3;
        self
    }
    pub fn GUILD_INTEGRATIONS(mut self) -> IntentsBuilder {
        self.intents = 1 << 4;
        self
    }
    pub fn GUILD_WEBHOOKS(mut self) -> IntentsBuilder {
        self.intents = 1 << 5;
        self
    }
    pub fn GUILD_INVITES(mut self) -> IntentsBuilder {
        self.intents = 1 << 6;
        self
    }
    pub fn GUILD_VOICE_STATES(mut self) -> IntentsBuilder {
        self.intents = 1 << 7;
        self
    }
    pub fn GUILD_PRESENCES(mut self) -> IntentsBuilder {
        self.intents = 1 << 8;
        self
    }
    pub fn GUILD_MESSAGES(mut self) -> IntentsBuilder {
        self.intents = 1 << 9;
        self
    }
    pub fn GUILD_MESSAGE_REACTIONS(mut self) -> IntentsBuilder {
        self.intents = 1 << 10;
        self
    }
    pub fn GUILD_MESSAGE_TYPING(mut self) -> IntentsBuilder {
        self.intents = 1 << 11;
        self
    }
    pub fn DIRECT_MESSAGES(mut self) -> IntentsBuilder {
        self.intents = 1 << 12;
        self
    }
    pub fn DIRECT_MESSAGE_REACTIONS(mut self) -> IntentsBuilder {
        self.intents = 1 << 13;
        self
    }
    pub fn DIRECT_MESSAGE_TYPING(mut self) -> IntentsBuilder {
        self.intents = 1 << 14;
        self
    }
    pub fn MESSAGE_CONTENT(mut self) -> IntentsBuilder {
        self.intents = 1 << 15;
        self
    }
    pub fn GUILD_SCHEDULED_EVENTS(mut self) -> IntentsBuilder {
        self.intents = 1 << 16;
        self
    }
    pub fn AUTO_MODERATION_CONFIGURATION(mut self) -> IntentsBuilder {
        self.intents = 1 << 20;
        self
    }
    pub fn AUTO_MODERATION_EXECUTION(mut self) -> IntentsBuilder {
        self.intents = 1 << 21;
        self
    }
    pub fn build(self) -> Intents {
        Intents {
            intents: self.intents,
        }
    }
}



pub struct IntentsV2 {}
impl IntentsV2 {
    pub const GUILDS: u64 = 1;
}