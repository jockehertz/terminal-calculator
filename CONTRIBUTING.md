**You are welcome to contribute!** More contributers are nothing but fun, there are however some things to think about:

## 1: Contact
Please contact the maintainers beforehand, to make sure that your contribution is not a complete surprise. It's also nice to get to know you contributors :).

## 2: Code of conduct
Most importantly of all, *follow the code of conduct*, defined in [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md). In there, you can read about the consequences of breaking the code of conduct.

## 3: Use our templates
To make sure that all the information we need is included, follow the templates for pull requests and issues when possible. You can find them in [.github/](.github/). 
If none of these fit your use case, it is of course better that something is written than nothing, but try to add labels to make it easier to understand what your issue/pull request is about at a glance.
It is also preferred to include brackets with a description of the subject in the title. For example: *[ENHANCEMENT POSSIBILITY] Possibility of enhancement in module ...*

## 4: Write clear and concise commits
When you make a commit, please describe the overall theme of the changes made. It should also be in past tense, so it makes more sense to read after the commit was made.

We do not need to see following:
 - What files had changes (this can be seen by looking at the blame)
 - An extensive background for why the changes were made (this should be in the pull request later)
 - When the commit was made (this can also be seen in the blame)

What we do like to see:
 - What general part of the application was changed (for example, *the parser*, not `parser.rs` as these could be spit into multiple files)
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
