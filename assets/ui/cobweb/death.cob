#import
buttons as _

#defs
+gene_button = \
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
    Animated<Width>{ idle:80px hover:85px press:85px }
    Animated<Height>{ idle:8px hover:10px press:10px }
    Animated<Margin>{
        idle:{top:1px bottom:1px}
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
        TextLineSize(7)
        TextLineFont({family:"Monogram"})
        Picking::Ignore
        Animated<TextLineColor>{ idle:#FFFFFF hover:#99e550 press:#99e550 }
\


#scenes
"scene"
    // Root node covers the window.
    FlexNode{
        width:100vw height:100vh
        justify_main:Center
        justify_cross:Center
        flex_direction:Column
    }

    "header"
        FlexNode{
            margin:{top:2px bottom:3px left:8px right:8px}
        }
        TextLine{text:"You have died!" font:{family:"Monogram"} size:23}


    "gene_menu"
        GridNode{
            height: 70%
            row_gap: 3px
            column_gap: 10px

            grid_auto_flow:Row
            grid_template_columns: [(Count(2), 100px)]
        }

        ""
            FlexNode{
                justify_main:Center
                justify_cross:Center
            }
            ""
                TextLine{text:"Inactive" font:{family:"Monogram"} size:12}

        ""
            FlexNode{
                justify_main:Center
                justify_cross:Center}
            ""
                TextLine{text:"Active" font:{family:"Monogram"} size:12}

        "inactive"
            GeneContainer::Inactive
            FlexNode{
                height:100%
                justify_main:FlexStart
                justify_cross:Center
                flex_direction:Column
            }

        "active"
            GeneContainer::Active
            FlexNode{
                justify_main:Center
                justify_cross:Center
                flex_direction:Column
            }


    ""
        +button{
            StateButton<Menu>(None)
            RespawnButton
            "text"
                TextLineText("Respawn")
        }


"gene_button"
    +gene_button{}
