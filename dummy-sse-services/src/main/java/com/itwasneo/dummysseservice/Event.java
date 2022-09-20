package com.itwasneo.dummysseservice;

public class Event {

	public long epoch;
	public String pair;
	public String close;

	public Event(long epoch, String pair, String close) {
		this.epoch = epoch;
		this.pair = pair;
		this.close = close;
	}
}
