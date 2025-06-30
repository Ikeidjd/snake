Collect the apples to gain score. If you hit yourself, you lose. If you touch the corners, you also lose, unless wrap_around is set to true.

WASD or arrow keys to turn the snake around. You can't turn 180 degrees (e.g.: pressing S while going up does nothing).<br>
F1 to toggle wrap_around. This restarts the game, so it only works if your score is 0 or if you're in the game over screen.<br>
R to restart. This only works if you're in the game over screen.

Your previous scores and high scores are saved on save.txt, one of each for no_wrap_around and one of each for wrap_around. The wrap_around flag also gets saved. See save_format.txt.<br>
The scores aren't updated if you got 0, since that essentially means you didn't really start the game.