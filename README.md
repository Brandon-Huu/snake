# snake

The snake game badly made in bevy.


## Mistakes ( that i will not fix )

- The snake and apple sprites are anchored by the center ( Anchor::Center )
  - I did not know that the Sprite component allowed you to change where the sprite is anchored from ( Default is Anchor::Center. I wanted Anchor::TopLeft ).
