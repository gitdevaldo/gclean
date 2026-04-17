# GClean ✉️

**Version:** `1.1`

**Say no to bounce emails. Forever.**

You've been there — you blast out a campaign, and half your emails bounce. Your sender reputation tanks, your ESP flags your account, and you're left wondering why you even bothered. GClean fixes that.

Before you send a single email, run your list through GClean and only keep the ones that are actually worth sending to. Fast, easy, and brutally honest about your email list.

---

## What does it do?

GClean takes your email list and tells you which ones are safe to send to and which ones aren't. That's it. No complicated setup, no technical knowledge needed. Just drop in your emails and get clean results back.

Whether you have 10 emails or 100,000, GClean handles it and gives you a clear answer for each one — valid or not valid.

---

## How we check

This isn't just a "does it have an @ sign" kind of check. GClean goes deep on every single email so you can actually trust the results.

**Format**
The obvious first step. If the email isn't even structured correctly, we catch it right away before wasting time on anything else.

**Domain existence**
A lot of fake emails use made-up domains. We verify the domain actually exists on the internet before going any further.

**MX records**
Having a domain is one thing, being set up to receive email is another. We check if the domain actually has mail servers configured behind it. No mail servers means no real inbox.

**Disposable and temp mail**
People use services like Mailinator, Guerrilla Mail, 10minutemail, and hundreds of others to sign up without giving their real email. We flag all of them so they don't pollute your list.

**Role-based addresses**
Addresses like `admin@`, `info@`, `support@`, and `noreply@` are technically valid but almost never belong to a real individual. They're shared inboxes, mailing lists, or bots. We flag these so you know what you're dealing with.

**SMTP verification**
We knock on the door of the actual mail server and ask if the mailbox exists, without sending a real email. A lot of addresses pass every other check but still don't have an active inbox behind them. This step catches those.

**Pwned check**
We cross-reference against known data breach databases. If the address appeared in a breach, it's a signal the email is real and active — useful to know either way.

**Catch-all domain detection**
Some domains are set up to accept literally every email sent to them, no matter what. So you can't fully trust a "valid" result on those domains. We detect and flag them so you're never caught off guard.

**Spam trap detection**
Spam traps are decoy addresses maintained by ISPs and anti-spam organizations to catch senders with dirty lists. Hitting one is a fast track to getting blacklisted. We check against known trap patterns so you never accidentally walk into one.

---

## Why GClean?

Because bounce rates are silent killers. Every bounce hurts your sender reputation, and once that's damaged it takes a long time to recover. Most people don't realize their list has a problem until it's already too late.

GClean gives you the confidence to send knowing your list is clean. No guessing, no hoping, no nasty surprises after you hit send.

And honestly? It's cheap. Way cheaper than the damage a bad list does to your deliverability.

---

## Input

| Field | Type | Required | Description |
|---|---|---|---|
| emails | Array | Yes | A JSON array of emails. Use one item for single validation or multiple items for bulk |

Set the actor secret in environment variables as:
`VALIDATION_API_TOKEN`

---

## Output

Each result row contains:
- `email`
- `status` (`true`, `false`, or `error`)

---

## Use cases

- Cleaning your email list before a campaign
- Validating signups in real time before they hit your CRM
- Scrubbing a purchased list before you use it
- Reducing bounce rates and protecting your sender score
- Keeping your ESP account healthy and out of trouble

---

## License

MIT — use it however you want.
