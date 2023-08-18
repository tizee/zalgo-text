# zalgo-text

A command-line tool that help convert input string to glitched text, which also known as zalgo text.

## Usage

I believe this tool can be used to generate zalgo-text test cases for any application with text rendering support. For example, terminal emulator, browser or other GUI application.

```
> zalgo-text "hello,world"
> h͖̫̍̂ͫe̶̞ͩ̈̎l͇͚ͯ̐ͫl̫̘̘̓͊o͉̐ͭ́͛,͓̓͏̥͝w̱̭ͨ͊̂o̠̬ͣ͡ͅŗͬͮ̇͡l̵̦̼̫ͣd̨̜̥͆͗
```

```
Usage: zalgo-text [OPTIONS] <TEXT>

Arguments:
  <TEXT>

Options:
  -n, --num <NUM>   mark num [default: 6]
      --extend      use extend marks
      --supplement  use supplement marks
      --symbol      use symbol marks
      --half        use half marks
  -h, --help        Print help
  -V, --version     Print version
```

## Motiviation

I was inspired by this tweet post [https://twitter.com/geekbb/status/1691815679618367752](https://twitter.com/geekbb/status/1691815679618367752).

## How it works

Use Unicode combining characters.
