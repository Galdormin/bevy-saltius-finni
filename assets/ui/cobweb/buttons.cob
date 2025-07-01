#defs
+button = \
    ControlRoot
    FlexNode{
        justify_main:Center
        justify_cross:Center
        justify_lines:Center
    }
    // BrRadius(15px)
    // Animated<BackgroundColor>{ idle:#007000 hover:#006200 press:#005500 }
    LoadedImageNode{
        image:"ui/button.png"
        atlas:{ alias:"atlas_button" index:0 }
        mode:Sliced({ border:{top:5 bottom:5 left:5 right:6} })
    }
    Animated<ImageNodeIndex>{ idle:0 hover:1 press:1 }
    Animated<Width>{ idle:70px hover:75px press:75px }
    Animated<Height>{ idle:13px hover:17px press:17px }
    Animated<Margin>{
        idle:{top:2px bottom:2px}
        hover:{top:0px bottom:0px}
        press:{top:0px bottom:0px}
    }

    // Sets up the button's text as a single line of text with margin to control the edges of the button.
    "text"
        ControlMember
        FlexNode{
            margin:{top:2px bottom:2px left:4px right:4px}
        }
        TextLine{}
        TextLineSize(11)
        TextLineFont({family:"Monogram"})
        Picking::Ignore
        Animated<TextLineColor>{ idle:#FFFFFF hover:#99e550 press:#99e550 }
\

#commands
LoadImages[
    "ui/button.png"
]
LoadTextureAtlasLayouts[
    {
        texture: "ui/button.png"
        alias: "atlas_button"
        tile_size: ( 40 27 )
        columns: 2
        rows: 1
    },
]
