---
title: Jellyfin setup
description: Setting up Jellyfin on Manatan
---

# Jellyfin setup
Manatan can play anime from your jellyfin library.

> ⚠️ **WARNING**
> 
> This guide does not cover how to install the Jellyfin extension. The extension should be included by default in future versions of the Manatan app.



## 1) Accessing your jellyfin

1. Open **Browse** > **Anime Extensions**.  
 ![Screenshot of Manatan's sidebar][IMG00]  
 ![Screenshot of Manatan's anime extensions list][IMG01]
2. Open the **source settings**.
 ![Screenshot of Manatan's anime extensions list with an arrow to the cog symbol of the jellyfin extension][IMG02]
3. The extension might support multiple instances of itself so you might have all your libraries organized, if thats the case select the settings of the desired source instance
 ![Screenshot of Jellyfin settings for instances][IMG03]
4. Fill the required fields to login, after successfully connecting to jellyfin you'll be able to select the library
 ![Screenshot of Jellyfin server settings][IMG04]
 ![Screenshot of Jellyfin library setting asking you to login][IMG05]
 ![Screenshot of Jellyfin library setting letting you select the target library][IMG06]

5. Select the desired library
 ![Screenshot of Jellyfin settings selecting the library][IMG07]
6. Now you can access the content from manatan!
 ![Screenshot of Jellyfin extension with content][IMG08]

## 2) Quality issues

When using the source it might be the case that the quality of the stream is bad no matter what.  
 ![4k Creative Commons video transcoded at 15Mbps][IMG09]

This is an effect of transcoding the content (this issue is not present on source quality).

There are two solutions for this problem

1. Disable **transcoding** straight from **jellyfins dashboard**.  
 ![Screenshot of transcoding disabled from jellyfins dashboard][IMG10]  
 ![4k Creative Commons video not transcoded (high quality)][IMG11]
2. Change the transcoding to h265 in the **source settings** (useful if you want to keep transcoding on in the server).
 ![Screenshot of the settings extensions to an arrow to the transcoding video codec setting][IMG12]
 ![Transcoding video codec setting with the text "h264"][IMG13]
 ![Transcoding video codec setting with the text "h265"][IMG14]
 ![4k Creative Commons video not transcoded (high quality)][IMG11]

## 3) Playback issues

There are instances where either certain or all videos can't be played, this can be often fixed with the same solutions discussed in Quality issues

1. Disable **transcoding** straight from **jellyfins dashboard**.  
 ![Screenshot of transcoding disabled from jellyfins dashboard][IMG10]  
2. Change the transcoding to h265 in the **source settings** (useful if you want to keep transcoding on in the server).
 ![Screenshot of the settings extensions to an arrow to the transcoding video codec setting][IMG12]
 ![Transcoding video codec setting with the text "h264"][IMG13]
 ![Transcoding video codec setting with the text "h265"][IMG14]

However there is a chance the issue might be caused by displaying missing episodes in jellyfin clients, so the solution naturally is to disable this feature

1. Disable **display missing episodes within seasons** from the user instance in a web browser from the **display settings**. 
 ![Screenshot of jellyfin web client display settings with an arrow to the "display missing episodes within seasons" checkbox][IMG15]  

If this still doesn't work:
2. Disable **display specials within seasons they aired in** from the **display settings for libraries** on the jellyfin **dashboard**.
 ![Screenshot of the settings extensions to an arrow to the transcoding video codec setting][IMG16]

## Notes
* At the moment of writting the source quality is not available on the app.
* At the moment of writting subtitles from the server do not work on the app.
* At the moment of writting the watch history is not syncronized into the jellyfin server and its completely local.
* If the errors that you are running into are not covered in this document please look into the github issues before reporting it.
## Tips

If you encounter any issues not covered in this document and its not a github issue please [Join the Discord](https://discord.gg/tDAtpPN8KK) for guidance and create the issue yourself.

## Next steps

- [Getting started](/docs/guides/getting-started)
- [Troubleshooting](/docs/guides/troubleshooting)

[IMG00]: /assets/jellyfin-setup/jellyfin-setup000.png
[IMG01]: /assets/jellyfin-setup/jellyfin-setup001.png
[IMG02]: /assets/jellyfin-setup/jellyfin-setup002.png
[IMG03]: /assets/jellyfin-setup/jellyfin-setup003.png
[IMG04]: /assets/jellyfin-setup/jellyfin-setup004.png
[IMG05]: /assets/jellyfin-setup/jellyfin-setup005.png
[IMG06]: /assets/jellyfin-setup/jellyfin-setup006.png
[IMG07]: /assets/jellyfin-setup/jellyfin-setup007.png
[IMG08]: /assets/jellyfin-setup/jellyfin-setup008.png
[IMG09]: /assets/jellyfin-setup/jellyfin-setup009.png
[IMG10]: /assets/jellyfin-setup/jellyfin-setup010.png
[IMG11]: /assets/jellyfin-setup/jellyfin-setup011.png
[IMG12]: /assets/jellyfin-setup/jellyfin-setup012.png
[IMG13]: /assets/jellyfin-setup/jellyfin-setup013.png
[IMG14]: /assets/jellyfin-setup/jellyfin-setup014.png
[IMG15]: /assets/jellyfin-setup/jellyfin-setup015.png
[IMG16]: /assets/jellyfin-setup/jellyfin-setup016.png