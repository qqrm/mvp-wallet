# SOURCE_2__Blueprint_v3_2

## Page 14

and resulting amount
. The user can confirm to execute, upon which the final result (updated
balances and a receipt showing the conversion details) is displayed
.
Transaction History: Users can view a list of past transactions (for each account). This is accessible
via a “History” tab. Transactions are listed with basic info (date, amount, type, counterparty or
description) and can be tapped to view the full receipt
. The history list uses lazy-loading
(cursor pagination) to fetch more records as the user scrolls. Filters (by type, date) are not in MVP but
planned later.
Receipts & Details: For any transaction (transfer, conversion, etc.), the user can view a detailed
receipt. This shows all relevant info: status, timestamp, from/to accounts (with names/contacts),
amounts, fees, exchange rate if applicable, and a unique transaction ID. The UI provides an option to
export or share this receipt (e.g. generate a PDF or image) for the user’s records
.
Admin Interface (MVP): There is no dedicated GUI for admins in MVP; admins use internal tools or
direct API calls (e.g. via Swagger or scripts) to perform admin operations. Admin-focused UI will be
developed post-MVP as noted.
Design & QA: The mobile UI follows Uzum’s design system for consistency. Standard components
(buttons, inputs, dialogs) are used. Biometric authentication or OTP confirmations are handled by
the existing app’s security (wallet actions assume the user is already logged in to the app). The wallet
features have been tested end-to-end in the app to ensure a smooth user experience (e.g. proper
loading states, error messages on failures like insufficient funds, etc.).
Sources: All information above was synthesized from the provided Uzum Wallet MVP Blueprint v3.1
and the v3.2 upgrade document
, ensuring that v3.2 enhancements (recipient resolution, static FX
rates, idempotency TTL, receipt details, Rust implementation, extended roadmap) were integrated into the
comprehensive specification. The comparative analysis of leading wallets
 informed the post-MVP
roadmap. This v3.2 blueprint is a single source of truth for the product and engineering teams moving
forward, superseding all prior versions.
uzum_wallet_mvp_blueprint_v3_1_landscape.pdf
file://file_00000000ad6071f49d1303084be7d5fa
Uzum Wallet MVP Blueprint (v3.2 Upgrade) – Contract-First
Monolith (Uzbekistan).pdf
file://file_00000000b0dc71f485b5d74f79762115
Comparative Analysis of Leading Wallet Apps and Uzum Wallet Backlog.pdf
file://file_00000000b59471f4bc330aa907762545
101
102
103
- 41
104
- 34
35
-
- 105
106
107
108
109
110
1
2
3
24
25
26
27
30
36
38
39
40
41
42
43
48
49
52
53
54
55
56
57
58
59
60
96
98
100
101
102
103
104
105
106
4
5
6
7
8
9
10
11
12
13
14
15
16
17
18
19
20
21
22
23
28
29
31
32
33
34
35
37
44
45
46
47
50
51
61
62
63
65
66
67
68
69
70
71
72
73
74
75
76
77
78
79
80
81
82
83
84
85
86
87
88
89
90
91
92
93
94
95
97
99
107
108
64
109
110
14
