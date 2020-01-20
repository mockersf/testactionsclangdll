extends Node2D

var StellarObject = preload("res://game/StellarObject.tscn")
var Player = preload("res://game/Player.tscn")

var player

func _ready():
	var new_star = StellarObject.instance()
	add_child(new_star)
	player = Player.instance()
	add_child(player)


func _process(delta):
	var speed = 100
	var velocity = Vector2()  # The player's movement vector.
	if Input.is_action_pressed("ui_right"):
	    velocity.x += 1
	if Input.is_action_pressed("ui_left"):
	    velocity.x -= 1
	if Input.is_action_pressed("ui_down"):
	    velocity.y += 1
	if Input.is_action_pressed("ui_up"):
	    velocity.y -= 1
	player.position += velocity * speed * delta
