

// setup
k=Scale.choose(7, 24).ratios;k = k ++ (k*2) ++ (k*3);k = k.scramble;


// run
(OSCdef(\o,{|a|{Pan2.ar(PulseDPW.ar(k[(a[1]*10)%k.size]*300*2)*
	EnvGen.ar(Env.perc(0.005,0.04),doneAction:2)*0.4,a[1]*2-1)}.play},'/'))