#commands
LoadImages[
    "ui/jump_icon.png"
]

#scenes
"hud"
    // Root node covers the window.
    FlexNode{
        margin: {left:2px, top:2px}
        width: 50px height: 20px
        justify_main:FlexStart
        justify_cross:Center
        flex_direction:Row
    }

    "icon"
        FlexNode{
            margin:{top:1px bottom:1px left:1px right:1px}
        }
        LoadedImageNode{
            image: "ui/jump_icon.png"
        }

    "time"
        FlexNode{
            margin:{top:1px bottom:1px left:1px right:1px}
        }
        TextLine{text:"x" font:{family:"m6x11"} size:13}

    "counter"
        FlexNode{
            margin:{top:1px bottom:1px left:1px right:1px}
        }
        JumpCounter
        TextLine{text:"5" font:{family:"m6x11"} size:13}
