# You are welcome to contribute!
More contributors are nothing but fun, there are however some things to think about.

When contributing to this community, we encourage you to have these values in mind:
 - Learning
 - Respect
 - Open-mindedness

## 1: Prior experience
Contributing to open source projects is in my (Joakim's) opinion, the single best way to learn coding. Therefore, you are more than welcome to contribute, even if you have experience of neither Rust nor programming itself. As a knowledgeable contributor who knows how to program, you have a responsibility to teach and encourage your fellow contributors. This is not a place for gatekeeping and flexing your programming skills on those less knowledgeable, but a place for learning, with a positive attitude towards questions. Remember: **There are no stupid questions**

## 2: Dictionary
If you are new to contributing, don't be discouraged! We have issue templates to get you started, and a [DICTIONARY.md](DICTIONARY.md) containing some basic phrases 
which are commonly used in open source projects.

## 3: Contact
Please contact the maintainers beforehand, to make sure that your contribution is not a complete surprise. It's also nice to get to know you as contributors :).

## 4: Code of conduct
Most importantly of all, *follow the code of conduct*, defined in [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md). In there, you can read about the consequences of breaking the code of conduct.

## 5: Use our templates
To make sure that all the information we need is included, follow the templates for pull requests and issues when possible. You can find them in [.github/](.github/). 
If none of these fit your use case, it is of course better that something is written than nothing, but try to add labels to make it easier to understand what your issue/pull request is about at a glance.
It is also preferred to include brackets with a description of the subject in the title. For example: *[ENHANCEMENT POSSIBILITY] Possibility of enhancement in module ...*

## 6: Write clear and concise commits
When you make a commit, please describe the overall theme of the changes made. It should also be in past tense, so it makes more sense to read after the commit was made.

We do not need to see the following:
 - What files had changes (this can be seen by looking at the blame)
 - An extensive background for why the changes were made (this should be in the pull request later)
 - When the commit was made (this can also be seen in the blame)

What we do like to see:
 - What general part of the application was changed (for example, *the parser*, not `parser.rs` as these could be split into multiple files)
 - What did you do? (In broad terms, e.g. refactored, extended, updated, fixed)
 - What did the changes do?

Example of a *good* commit message:
```
Refactored generate_ast() in the parser for improved clarity.
```

- What general part of the application was changed? - The `generate_ast` function in the parser.
- What did you do? - Refactor the code.
- What did the changes do? - Improve clarity.

If your changes required small changes in other files, these do *not* need to be included, that is better seen in the blame when reviewing a pull request.

## 7: Branches and versions
Each version has a milestone in GitHub projects. Since the base  of the calculator is now working, development of each new version will happen on their respective branch. For example, all development for version 0.6.0 is pushed to the branch `v-0.6.0`. When the milestone for version 0.6.0 has been achieved, `v-0.6.0` is merged into `main` for production. 

Please base your contributions on the current version branch. To avoid confusion, all previous version branches will be archived, and the current one will be the active branch.

To contribute to the current version branch, develop on your own branch until you have ensured the intended functionality, then open a pull request against the current version branch. If you need help with your development, we recommend you to open an issue with the *help-wanted* label. Remember to push your code to your branch so other contributors can see the code, and specify which branch you are working on.

The workflow specified above is *always* used for contributions, even by maintainers.

# Welcome, and happy contributing!