class Menu(object):
    running = True
    def main(self, screen):
        clock = pygame.time.Clock()
        menu = kezmenu.KezMenu(
            ['Play!', lambda: Game().main(screen)],
            ['Quit', lambda: setattr(self, 'running', False)],
        )
        menu.position = (50, 50)
        menu.enableEffect('enlarge-font-on-focus', font=None, size=60, enlarge_factor=1.2, enlarge_time=0.3)
        menu.color = (255,255,255)
        menu.focus_color = (40, 200, 40)

        nightmare = construct_nightmare(screen.get_size())
        highscoresurf = self.construct_highscoresurf()

        timepassed = clock.tick(30) / 1000.

        while self.running:
            events = pygame.event.get()

            for event in events:
                if event.type == pygame.QUIT:
                    exit()

            menu.update(events, timepassed)

            timepassed = clock.tick(30) / 1000.

            if timepassed > 1: # A game has most likely been played 
                highscoresurf = self.construct_highscoresurf()

            screen.blit(nightmare, (0,0))
            screen.blit(highscoresurf, highscoresurf.get_rect(right=WIDTH-50, bottom=HEIGHT-50))
            menu.draw(screen)
            pygame.display.flip()

    def construct_highscoresurf(self):
        font = pygame.font.Font(None, 50)
        highscore = load_score()
        text = "Highscore: {}".format(highscore)
        return font.render(text, True, (255,255,255))

def construct_nightmare(size):
    surf = Surface(size)

    boxsize = 8
    bordersize = 1
    vals = '1235' # only the lower values, for darker colors and greater fear
    arr = pygame.PixelArray(surf)
    for x in range(0, len(arr), boxsize):
        for y in range(0, len(arr[x]), boxsize):

            color = int(''.join([random.choice(vals) + random.choice(vals) for _ in range(3)]), 16)

            for LX in range(x, x+(boxsize - bordersize)):
                for LY in range(y, y+(boxsize - bordersize)):
                    if LX < len(arr) and LY < len(arr[x]):
                        arr[LX][LY] = color
    del arr
    return surf
