# phrase

A passphrase generator: no-config, 128-bit security, memorable, & built-in simplified English word list.

## A no-config manifesto.

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
