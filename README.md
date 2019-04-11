# inline-test

```
$ cargo build --release
$ ./disassemble

204005ee:	3240      	adds	r2, #64	; 0x40
204005f0:	4572      	cmp	r2, lr
204005f2:	d3d9      	bcc.n	204005a8 <Reset+0x1a8>
204005f4:	bf30      	wfi
204005f6:	bf30      	wfi
204005f8:	bf30      	wfi
204005fa:	bf30      	wfi
204005fc:	bf30      	wfi
204005fe:	e7fe      	b.n	204005fe <Reset+0x1fe>
20400600:	f101 0610 	add.w	r6, r1, #16
20400604:	2802      	cmp	r0, #2
```
