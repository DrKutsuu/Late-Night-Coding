program Mordor;

uses crt, math;

var
SceneryX : integer;
SceneryY : integer;
LetterA  : string;
LetterB : string;
LetterC : string;
LetterD : string;
LetterE : string;
LetterF : string;              
turns : integer;

procedure fillupscreen;
 begin
  gotoxy (SceneryX,SceneryY);
  write (LetterA);
  gotoxy (SceneryX+1,SceneryY+1);
  write (LetterB);
  gotoxy (SceneryX+2,SceneryY+2);
  write (LetterC);
  gotoxy (SceneryX+3,SceneryY+3);
  write (LetterD);
  gotoxy (SceneryX+4,SceneryY+4);
  write (LetterE);
  gotoxy (SceneryX+5,SceneryY+5);
  write (LetterF);
  textcolor (randomrange (1,112));
  SceneryX := randomrange (1,200);
  SceneryY := randomrange (1,200);
 end;

BEGIN
 clrscr;
 randomize;
  SceneryX := randomrange (1,200);
   SceneryY := randomrange (1,200);
  LetterA := ('....>');
 LetterB := ('^ ....');
  LetterC := ('0....');
 LetterD := ('1....');
  LetterE := ('X');
 LetterF := ('/');
 turns := (1);
 while turns = (1) do
     begin
     fillupscreen;
     end;

END.

