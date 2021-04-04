# DS Map Application
This is a simple key/value map that implements [ds-libs](https://github.com/CoffmanTaylor/DS-libs)'s `Application` trait.

# Operations
It currently allows for three operations:
- Get(`key`)
- Store(`key`, `value`)
- Clear(`key`)

The next version should also support an non-idempotent update command. Useful for testing. But the current version does not have it and I'm not allowed to add it until im done writing the READMEs.