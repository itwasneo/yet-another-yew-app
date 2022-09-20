package com.itwasneo.dummysseservice;

import io.javalin.Javalin;
import io.javalin.http.sse.SseClient;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.Timer;
import java.util.TimerTask;
import java.util.concurrent.ConcurrentLinkedQueue;

public class DummySSEService {
	private static final ConcurrentLinkedQueue<SseClient> clients = new ConcurrentLinkedQueue<>();

	private static final Logger logger = LoggerFactory.getLogger(DummySSEService.class);

	public static void main(String[] args) {

		Javalin app = Javalin.create(conf -> {
			conf.enableCorsForAllOrigins();
		});

		app.sse("/sse", client -> {
			clients.add(client);
			client.onClose(() -> clients.remove(client));
		});

		Timer t = new Timer();
		t.scheduleAtFixedRate(
				new TimerTask() {
					@Override
					public void run() {
						for (SseClient client : clients) {
							logger.info("client: {}", client);
							try {
								Thread.sleep(Long.parseLong(args[1]));
							} catch (InterruptedException e) {
								throw new RuntimeException(e);
							}
							client.sendEvent(new Event(
									System.currentTimeMillis(),
									"BTCUSDT",
									"19000"));
						}
					}
				}, 0, 5000
		);
		app.start(Integer.parseInt(args[0]));
	}
}
