extends Control

var Star = preload("res://menu/Star.tscn")

var star_percent = 0.0007
var rng = RandomNumberGenerator.new()

var star_colors = [
	[0.7, Color(0.7, 0.7, 0.7, 0.6)],
	[0.8, Color(1, 0.6, 0.6, 0.6)],
	[0.9, Color(1, 1, 0.6, 0.6)],
	[1.0, Color(0.6, 0.6, 1, 0.6)],
]

func _ready():
	rng.randomize()

func _process(delta):
	var size = get_size()
	var nb_stars = get_node("Background/Stars").get_child_count()

	# create new stars if some are missing
	while nb_stars < (size[0] * size[1] * star_percent):
		var new_star = Star.instance()
		new_star.position = Vector2(rng.randi_range(0, size[0]),rng.randi_range(0, size[1]))

		var color_sel = rng.randf()
		var star_color = star_colors[0][1]
		for color in star_colors:
			if color[0] < color_sel:
				star_color = color[1]
		new_star.get_node("Star").color = star_color

		get_node("Background/Stars").add_child(new_star)

		nb_stars += 1
