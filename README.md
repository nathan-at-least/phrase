# phrase

A passphrase generator: no-config, 128-bit security, memorable, & built-in simplified English word list.

## Design

The tool prints a randomly generated passphrase of 128 bit security. That's it.

The "design opinion" is `no-config` described below. There are no options or config files. Words are selected from a hard-coded list and printed until at least 128 bits of entropy have been printed. There is a constant prefix that includes a capital, number, and symbol to account for dumb password restrictions. These don't provide any of the security and they don't need to, they are just for bypassing unecessary-yet-popular restrictions on password format.

If you need to encrypt the password, pipe it to an encryption program.

If you need a shorter password, just copy a shorter prefix or pipe to a command to truncate it.

If you can't use spaces, pipe it to a command to replace/remove spaces.

If you need a different word list, a different seed, to generate passphrases that fit unique password restriction criteria, or many other features, there are many alternative tools that do those things. This one keeps it simple. ;-)

## A no-config manifesto

No-config is a design opinion that's broader than configuration per se: it's about making code do a narrow, very specific, yet useful thing. Having no configuration and few if any commandline parameters are signs of a good no-config design.

An idealistic heuristic is to avoid adding any functionality unless we're confident >50% of users will need it.

Let's say 30% of users need a feature that the code doesn't have. Can they band together to create a new tool addresses their need? Can it compose cleanly with the no-config app without complicating the dependency? If yes to both, clear win. If yes only to the first, still a win in my book.

### Why no-config?

It should work perfectly for a large frequent use case.

For other cases use other tools, or perhaps compose tools in the unix way (for example pipe through `sed` or `rage`).

This way, the devs and all of the users converge on a very clear understanding of the behavior, requirements, and code.

### for users

Users will learn the behavior and limits of the code more readily. Users who have less common use cases will either use use the code inside their own flow that solves their problem, or they will use a different tool.

The users who've made their own solution can make sure that solution works for them, and if it doesn't work, they're better equipped to figure out why. They're less likley to need to go back and forth with an open source volunteer diagnosing an issue that the no-config tool doesn't support.

Meanwhile, they also haven't interfered with all the other users by pushing their less common usecase complexity to all users, so the tool is more likely to support the narrower use case more effectively.

### for devs

There is less code, so more maintainers and code-literate users are likely to have a more comprehensive and shared understanding of the code.

It is more likely to be tested by all users, because the only people who use it are more likley to be using it in similar ways.

It is easier to compose with other code or deployments because the behavior is more well understood. This is "getting back to roots of the unix way".

## Acknowledgements

The word list, currently, comes from [pyutil](https://github.com/zooko/pyutil). It was selected due to the author's familiarity of the word list's design which I suspect of having better memorability than many diceware-like alternatives. It uses canonicalized words (stem form), avoiding things like plurals or possessives or verb cases. It excludes proper nouns. It is also hand filtered a bit. The goal is to keep the word choices simple to remember and spell for native English speakers (maybe with a U.S. English bias).
