# Setup Kiku for Manatan 
Kiku is a fully interactive Anki note type designed for Japanese learners. For more info please refer to the [official website](https://kiku.youyoumu.my.id/).

```WARNING: Anki 25.09 or later is required.```

## 1) Installing the note type
Download the latest release ```Kiku_v*.apkg``` from [Releases](https://github.com/youyoumu/kiku/releases/latest), and then import it to your Anki.

## 2) Setting up anki fields
Open Manatan, go to ```Manatan settings``` and make sure the ```Enable AnkiConnect``` setting is turned on. 

You will see the text ```Connected``` in green letters.

Select your desired `Target Deck` and select ```Kiku``` as the `Card Type`, then configure the following fields:


| Anki Field                 | Content                                                                                                                                                      |
| --------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Expression            | `Target Word`                                                                                                                                             |
| ExpressionFurigana    | `Furigana`                                                                                                                                         |
| ExpressionReading     | `Reading`                                                                                                                                                |
| ExpressionAudio       | `Word Audio`                                                                                                                                                  |
| SelectionText         |                                                                                                                                   |
| MainDefinition        | Something like `Single Glossary JMdict`. Find this by clicking the down arrow next to this field, and finding a dictionary in a similar format. |
| DefinitionPicture     |                                       |
| Sentence              | `Sentence with Bold Word`                                                                                                          |
| SentenceFurigana      |                                                                                                                                                            |
| SentenceAudio         | `Sentence Audio`                                                                                                                                                           |
| Picture               | `Image`                                                                                                                                                           |
| Glossary              | `Glossary`                                                                                                                                               |
| Hint                  |                                                                                                                                                            |
| IsWordAndSentenceCard |                                                                                                                                                            |
| IsClickCard           |                                                                                                                                                            |
| IsSentenceCard        |                                                                                                                                                            |
| IsAudioCard           |                                                                                                                                                            |
| PitchPosition         | `Pitch Accent Positions`                                                                                                                                 |
| PitchCategories       | `Pitch Accent Categories`                                                                                                                        |
| Frequency             | `Frequency`                                                                                                                                            |
| FreqSort              | `Harmonic Frequency`                                                                                                                                |
| MiscInfo              | `Media Name`    |

Credit to [Lapis](https://github.com/donkuri/lapis) for the table.

## Need help?

If setup does not work, ask in Discord and include:

- your platform
- what you expected
- what happened instead

- [Join the Discord](https://discord.gg/tDAtpPN8KK)
- [Troubleshooting](/docs/guides/troubleshooting)
