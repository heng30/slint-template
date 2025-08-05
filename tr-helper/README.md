A tool to extract all sentences, needed to be translated, of current project.

----

### How to use?
Enter the Rust workspace directory, and then run `make tr` to extract all sentences that need to be translated.

----

### The prompt to translate a sentence from English to Chinese
按行翻译上面的英文。大小写敏感，即使相同的单词也要进行翻译。输出格式如下：
 ```
 ("英文", "翻译的中文"),
 ```
