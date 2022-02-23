# Rust Adventure
A small text adventure written in Rust.

## Motivation
It was fun writing this.  I used to love Zork, playing on a DEC PDP-11.  
My son was recently making a "choose your own adventure" in the programming environment: Scratch.  
So, per usual, I had to do my own challenge, proportionately difficult to his challenge.  
So I chose to do this in the Rust Programming Language 
-- not to be confused with the Rust survival video game.

## Scope
In its current state, it has a definitive ending. 
It should take maybe 20 minutes.  
You can eat and smell things, but that is just for fun 
and to see how I could parse a vector of words using a match statement.  
In one room, it tells you to look at something -- otherwise it's just: 

    east, west, north, south, and inventory.  

I had a ball making colors using a crate for terminal codes.

## Extensibility
Also, maybe here, maybe a new project, but it would be cool to make it extensible.  
By being in github it is extensible -- but let me explain:

When I was a kid there was a code project in Creative Computing that was a text adventure 
where you could add new rooms while you played.  
You would type "add room" and it would prompt for a new direction from the current room, 
then prompt for a name, description, and directions (for example: east, west) for the next 
connected rooms. It would save all this and immediately become a new room in the game.  
I think it even allowed you to place items in the rooms that could be taken into your inventory.

This was back when "network" was a group of friends standing around a computer.   
I think it would be interesting to create such a game and open it up to the internet to be played, 
and extended simultaneously by many people -- like a wiki.  Now that I think about it, 
it's probably already done, I have not searched.  I'll bet it has not been done using Rust!

I did search for a crate made for defining a text adventure.  None!

So, I'm happy to receive pull requests if you want to extend the adventure the github way.

david.