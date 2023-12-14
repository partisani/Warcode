create_player:
	x,y,hp
create_health:
	amount

create_player:
	x,y,hp
create_health:
	amount

create_player:
	lda 0
sta $0; x
lda 0
sta $0; y
lda 0
sta $0; hp
create_health:
	lda 0
sta $0; amount

