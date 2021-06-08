# Troubleshooting
dmarc-receiver depends hugely on opensmtp, and how it has been configured.
Since the configuration should be supplied by the user, this is the most probable place where troubleshooting is needed.

### no rules, nothing to do
You did not supply `smtpd.conf`. Don't forget to mount your configuration to `/etc/smtpd/smtpd.conf`.
