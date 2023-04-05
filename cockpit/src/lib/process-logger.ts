export interface Log {
	msg: string;
	level: LogLevel;
	file?: string;
	line?: number;
}

export enum LogLevel {
	/**
	 * The "error" level.
	 * Designates very serious errors.
	 */
	ERROR = 1,
	/**
	 * The "warn" level.
	 * Designates hazardous situations.
	 */
	WARN,
	/**
	 * The "info" level.
	 * Designates useful information.
	 */
	INFO,
	/**
	 * The "debug" level.
	 * Designates lower priority information.
	 */
	DEBUG,
	/**
	 * The "trace" level.
	 * Designates very low priority, often extremely verbose, information.
	 */
	TRACE
}

export function logLevelLabel(level: LogLevel) {
	switch (level) {
		case LogLevel.ERROR:
			return 'Error';
		case LogLevel.WARN:
			return 'Warning';
		case LogLevel.INFO:
			return 'Info';
		case LogLevel.DEBUG:
			return 'Debug';
		case LogLevel.TRACE:
			return 'Trace';
	}
}

export class ProcessLogger {
	private processLogSocket: WebSocket | undefined;

	constructor(public readonly address: string) {}

	start() {
		return new Promise<ReadableStream<Log>>((resolve, reject) => {
			if (this.processLogSocket) return;

			this.processLogSocket = new WebSocket(this.address);

			this.processLogSocket.addEventListener('error', reject);

			this.processLogSocket.onopen = () => {
				const stream = new ReadableStream<Log>({
					start: controller => {
						this.processLogSocket.addEventListener(
							'message',
							msg => {
								controller.enqueue(JSON.parse(msg.data));
							}
						);
					}
				});

				resolve(stream);
			};
		});
	}

	stop() {
		this.processLogSocket?.close();
		this.processLogSocket = undefined;
	}

	async restart() {
		this.stop();
		await this.start();
	}
}
