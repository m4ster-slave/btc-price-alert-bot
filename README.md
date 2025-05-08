# BTC Price Alert Bot

A simple Rust application that monitors Bitcoin prices and sends Discord alerts when the price drops below a specified threshold.

## Features

- Real-time Bitcoin price monitoring via the Blockchain.info API
- Customizable price alert thresholds
- Discord webhook integration with detailed embeds
- Configurable monitoring frequency and alert timeout
- Environment variable configuration

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (1.56.0 or later)
- A Discord server with webhook permissions

## Installation

1. Clone this repository:

   ```
   git clone https://github.com/m4ster-slave/btc-price-alert-bot.git
   cd btc-price-alert-bot
   ```

2. Create a `.env` file in the project root with the following variables:

   ```
   ALERT_PRICE=70000    # Price threshold for alerts (in USD)
   CHECK_TIME=900       # Time between price checks (in seconds)
   ALERT_TIMEOUT=86400  # Minimum time between alerts (in seconds)
   WEBHOOK="https://discord.com/api/webhooks/your-webhook-url"
   ```

3. Build the project:
   ```
   cargo build --release
   ```

## Configuration

### Environment Variables

| Variable        | Description                                                                       | Default          |
| --------------- | --------------------------------------------------------------------------------- | ---------------- |
| `ALERT_PRICE`   | The BTC price threshold (in USD) that triggers an alert when price drops below it | 70000            |
| `CHECK_TIME`    | How often to check the Bitcoin price (in seconds)                                 | 900 (15 minutes) |
| `ALERT_TIMEOUT` | Minimum time between alerts (in seconds)                                          | 86400 (24 hours) |
| `WEBHOOK`       | Discord webhook URL                                                               | _Required_       |

## Usage

Run the compiled binary:

```
./target/release/btc-price-alert-bot
```

For continuous operation, consider using a process manager like systemd, supervisor, or running it in a Docker container.

## How It Works

1. The application fetches the current Bitcoin price in EUR from the Blockchain.info API
2. If the price falls below your specified threshold, it sends a Discord alert via webhook
3. After sending an alert, it waits for the duration specified in `ALERT_TIMEOUT` before sending another
4. The application continuously monitors prices at intervals specified by `CHECK_TIME`

## Discord Alert Format

When triggered, the bot sends a Discord message with:

- A mention to @here
- An embed containing:
  - Current BTC price
  - Your alert threshold
  - Timestamp of the alert
  - Color-coded red for visibility

## License

[GNU GPLv3](LICENSE)

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
