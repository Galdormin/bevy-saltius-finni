#import
buttons as _

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
            margin:{top:8px bottom:8px left:8px right:8px}
        }
        TextLine{text:"You have died!" font:{family:"Monogram"} size:23}


    "gene_menu"
        GridNode{
            height: 60%
            row_gap: 3px
            column_gap: 10px

            grid_auto_flow:Row
            grid_template_columns: [(Count(2), 100px)]
        }

        "inactive"
            GeneContainer::Inactive
            FlexNode{
                justify_main:Center
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
            ChangeMenuButton(None)
            EventButton(Respawn)
            "text"
                TextLineText("Respawn")
        }
