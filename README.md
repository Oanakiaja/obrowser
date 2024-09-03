# O browser engine

> a mini browser engine for learning browser arch

Browser Process
* UI thread

Render Process：
* main thread
  * [finished] layout engine： based on https://limpet.net/mbrubeck/2014/08/08/toy-layout-engine-1.html.
    * [todo] layer tree 
  * [optional] javascript interpreter
* compositor thread
* raster thread


I plan to learn Rust & Cpp & Browser Engine from this base project. So I create this repo. 

There are still amounts of features that need to be implemented.


# Result 

https://oanakiaja.github.io/layout-engine-toy/

![result](./result.png)



