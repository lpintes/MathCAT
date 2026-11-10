# Slovak spoken-math terminology: sources

Research done 2026-11-10 for the Slovak (sk) MathCAT speech rules.

## Summary

No Slovak document was found that defines how to read whole mathematical
expressions aloud for blind users. No Slovak equivalent of ClearSpeak or
MathSpeak turned up, and no reading rules from the Levoča library or the
schools for the blind. What exists:

- authoritative Slovak names for individual symbols (the 2021 Slovak Braille
  rules from SABP and the NVDA Slovak symbol dictionary);
- one Slovak description of spoken output for structures (fraction start,
  fraction bar, end of fraction), from the Slovak localization of the Lambda
  editor;
- school and university materials that state readings for powers, roots,
  fractions, derivatives and definite integrals.

Spoken readings for limits, sums, absolute value, functions and indices
(subscripts) have no explicit Slovak source. Those entries below are marked
as unsourced suggestions.

## 1. Sources

- SABP 2021, Braille rules. "Pravidlá písania a používania Braillovho písma
  v Slovenskej republike" (rules for writing and using Braille in the Slovak
  Republic). Published by Slovenská autorita pre Braillovo písmo (SABP), the
  Slovak Braille Authority at Slovenská knižnica pre nevidiacich Mateja
  Hrebendu (SKN), Levoča, 2021, ISBN 978-80-18-10144-5.
  URL: http://liblouis.io/braille-specs/slovak/pravidla-bp01.pdf
  - What it covers: Braille codes with a "slovný popis" (verbal description)
    column for each symbol. That includes basic operators and relations
    (section 13.2), Greek letters, percent, degree, prime, brackets and units.
    It also has spoken examples of numbers and formulas ("pí krát r na druhú",
    "tri celých štrnásť stotín").
  - Trust: high. It is the official national standard body for Braille, and
    its descriptions name symbols as Slovak blind readers learn them. It only
    covers "basic technical notation"; a fuller math part is announced but
    was not found online.
- NVDA Slovak symbol dictionary, `source/locale/sk/symbols.dic` in
  nvaccess/nvda. Checked in the local clone
  `C:\vcs\github.com\nvaccess\nvda`, last commit to the file 84d079ccf
  (2025-11-25).
  URL: https://github.com/nvaccess/nvda/blob/master/source/locale/sk/symbols.dic
  - What it covers: Slovak names for about 150 math symbols (relations, set
    operators, logic, integrals, roots, vulgar fractions, number sets,
    arrows).
  - Trust: medium. Slovak users of the most common screen reader hear these
    names every day, so they are familiar. But they come from community
    translation, some are literal translations of Unicode names, and a few are
    wrong. For example, ℍ is given as "zlomky" (fractions), 𝕁 as
    "kladné (celé) čísla", and "≈" as "takmer úplne sa rovná".
- Horňanský 2009, student thesis on the Lambda editor. Martin Horňanský,
  "Sprístupňovanie matematiky nevidiacim na Slovensku prostredníctvom editora
  Lambda" (making math accessible to blind people in Slovakia with the Lambda
  editor). Študentská vedecká konferencia (student research conference)
  2009, FMFI UK Bratislava; supervisor I. Kohanová. It documents the Slovak
  localization of the Lambda editor, made at the UK Support Centre (Podporné
  centrum) with Peter Lecký.
  URL: https://www.duxburysystems.org/codes/slovak_math.pdf
  - What it covers: spoken output of Lambda for compound fractions (table
    4.2), set operator names, the notion of "koniec odmocniny" (end of root),
    and a Slovak rendering of the Aster/MathTalk grouping example.
  - Trust: medium. It is a student work, but it is the only Slovak source
    found that describes spoken structural markers tested with blind
    students.
- vypocitat.sk, "Mocniny a odmocniny (kalkulačka)" (powers and roots
  calculator).
  URL: https://vypocitat.sk/mocniny-a-odmocniny-kalkulacka/
  - What it covers: the reading of a² and a³, and the phrase "druhá
    odmocnina z a".
  - Trust: medium-low. A popular educational site, but its readings match
    common school usage.
- Rebelova škola doma, "Mocniny a odmocniny" (powers and roots).
  URL: https://rebelovaskoladoma.estranky.sk/clanky/matika/mocniny-a-odmocniny.html
  - What it covers: the rule that the exponent is read as an ordinal in the
    accusative case.
  - Trust: medium-low (a home-schooling site). It is the only source found
    that states the grammatical rule explicitly.
- Pohodová matematika (Rudolf Zrebný), "Čo sú zlomky? Čítanie a zapisovanie
  zlomkov" (what fractions are; reading and writing them).
  URL: https://pohodovamatematika.sk/co-je-zlomok-citanie-a-zapisovanie-zlomkov.html
  - What it covers: reading fractions in words, and the "lomeno" form.
  - Trust: medium-low (personal educational site by a teacher). Useful
    because it discusses both reading styles.
- Slovak Wikipedia, articles "Mocnina", "Zlomok (matematika)" and "Limita".
  URLs: https://sk.wikipedia.org/wiki/Mocnina ,
  https://sk.wikipedia.org/wiki/Zlomok_(matematika) ,
  https://sk.wikipedia.org/wiki/Limita
  - What it covers: names of the parts (základ, exponent, mocniteľ, čitateľ,
    menovateľ, zlomková čiara), one reading of a^b, and the verbal form of a
    limit statement. The "Odmocnina" article has no reading.
  - Trust: medium for terminology. Little on spoken readings.
- encyklopedia.sk (Biblioteka.sk, a mirror of Slovak Wikipedia), "Derivácia
  (funkcia)" (derivative).
  URL: https://www.encyklopedia.sk/encyklopedia/?pojem=Deriv%C3%A1cia_%28funkcia%29
  - What it covers: the reading of the Leibniz notation.
  - Trust: medium.
- TUKE "Bach-Math", integral page by Miriama Drančáková, LF TUKE (Technical
  University of Košice), 2021.
  URL: https://peter.szabo.website.tuke.sk/bach-math/pages/integral.html
  - What it covers: the verbal description of a definite integral and of dx.
  - Trust: medium (university teaching material).
- Školská pomôcka (school worksheet), "Pojem druhá mocnina ..." (the notion
  of a square), ZŠ Skýcov, 2017.
  URL: https://skolaskycov.weblahko.sk/matematika/9_mocniny.pdf
  - What it covers: school terminology for powers and roots (druhá, tretia,
    n-tá mocnina and odmocnina; exponent = mocniteľ; základ odmocniny; znak
    odmocnenia).
  - Trust: medium for terms, at the level of grade 9 school use.
- mathematikos.sk, "Matematicko-slovenský slovník" (a math-to-Slovak
  dictionary).
  URL: https://mathematikos.sk/matematicko-slovensky_slovnik
  - What it covers: an informal explanation of the sum sign.
  - Trust: low. No reading of the full expression.
- Czech, secondary: Jedličková, Krupka, Nechvátalová, "Matematika – Výrazy a
  rovnice 1" (expressions and equations), Nová škola, 2016, an approved
  Czech textbook.
  URL: https://www.nns.cz/blog/wp-content/uploads/Matematika-Výrazy-a-rovnice-1.pdf
  - What it covers: explicit Czech readings of powers of fractions and of
    negative numbers, including the "to celé" (all of it) grouping phrase.
  - Trust: high for Czech. Only an analogy for Slovak.
- Czech, secondary, background only: Teiresiás, Masaryk University Brno.
  The poster "Technologie pro zpřístupnění výuky matematiky zrakově
  postiženým na MU" (technologies for making math teaching accessible to
  visually impaired students at MU).
  URL: https://www.teiresias.muni.cz/formin/documents/matematika%20final.pdf
  - What it covers: only the tools (BUF, Lambda, BlindMoose). No spoken
    readings.
- Checked with no useful result:
  - STN EN ISO 80000-2:2020. The normoff.gov.sk preview (record 130359)
    shows that the Slovak edition contains only the English text, so there
    are no official Slovak verbal equivalents from ISO 80000-2. The 2017
    edition and STN ISO 31-11 previews returned 404.
  - Access8Math (NVDA add-on) has no Slovak locale.
  - SKN Levoča web pages: no published rules for reading math in audio
    books.

## 2. Slovak readings found

Format: "English construct: Slovak reading (source)". "Unsourced suggestion"
marks an entry that no source states.

### Fractions

- Fraction, named parts: čitateľ, menovateľ, zlomková čiara (sk.wikipedia
  Zlomok)
- Compound fraction, spoken structure: "zložený zlomok" at the start
  (alternatively just "zlomok"), "zlomková čiara" in the middle, "koniec
  zlomku" at the end. Example: "Zložený zlomok a plus b Zlomková čiara a
  mínus b Koniec zlomku." (Horňanský 2009, table 4.2, Lambda Slovak voice
  output)
- Simple numeric fraction 1/2: jedna polovina; then dve, tri, štyri poloviny;
  päť, šesť ... polovín (Pohodová matematika)
- Other denominators: tretiny, štvrtiny, pätiny ... stotiny (Pohodová
  matematika)
- Fraction with a large denominator, for example 3/58: "tri lomeno
  päťdesiatosem", or "tri osem a päťdesiatiny" (Pohodová matematika)
- Vulgar fraction characters: ½ polovica, ⅓ tretina, ⅔ dve tretiny, ¼
  štvrtina, ¾ tri štvrtiny, ⅕ pätina, ⅚ päť šestín, ⅛ osmina, ⅜ tri osminy
  (NVDA sk symbols.dic). Note that NVDA says "polovica" while Pohodová
  matematika says "polovina".
- Fraction bar ∕: zlomková čiara; slash /: lomka (SABP 2021). NVDA reads ∕
  as "deleno" and / as "lomka".
- Division ÷: deleno (SABP 2021; NVDA)

### Powers

- a²: "a na druhú"; a³: "a na tretiu" (vypocitat.sk)
- a^b: "a na b-tu"; the result is called the "b-tá mocnina čísla a"
  (sk.wikipedia Mocnina)
- General rule: base, then "na", then the exponent as an ordinal number in
  the accusative case. "2⁶ čítame ako dva na šiestu" (Rebelova škola doma)
- In a formula: πr² is read "pí krát r na druhú" (SABP 2021, example
  "výpočet obsahu", area calculation)
- Parts: základ (mocniny) / mocnenec; exponent / mocniteľ (sk.wikipedia
  Mocnina; ZŠ Skýcov worksheet)
- Squared units: km² "kilometrov štvorcových" (SABP 2021)
- Czech analogue: (2/5)⁴ "dvě pětiny, to celé na čtvrtou"; 2⁴/5 "2 na
  čtvrtou lomeno pěti"; (−2)⁵ "minus dvě, to celé na pátou" (Czech, Nová
  škola 2016)
- Grouped power, Slovak form "to celé na ...": unsourced suggestion, by
  analogy with the Czech "to celé".

### Roots

- √a: "druhá odmocnina z a" (vypocitat.sk). √ alone: "druhá odmocnina"
  (NVDA)
- ∛ tretia odmocnina; ∜ štvrtá odmocnina (NVDA; it spells "štvrta" without
  the accent, which is a typo)
- n-th root: "n-tá odmocnina z čísla x" (sk.wikipedia Odmocnina, used in the
  definition); "n-tá odmocnina" (ZŠ Skýcov worksheet)
- Parts: odmocniteľ (the index); základ odmocniny (the radicand); znak
  odmocnenia (the root sign) (ZŠ Skýcov worksheet)
- End-of-root marker exists as a concept, "koniec odmocniny" (Horňanský
  2009). The spoken string itself is not given.

### Indices (subscripts and superscripts)

- Braille markers: začiatok horného indexu (start of superscript), začiatok
  dolného indexu (start of subscript), koniec indexu (end of index) (SABP
  2021, section 13.1)
- Prime ′: indexová čiarka; ″ dvojitá indexová čiarka (NVDA). SABP 2021 has
  "indexová čiarka, uhlová ..." (the second word is truncated in the
  extracted text).
- Spoken subscript x₁ as "x jedna" or "x index jedna": unsourced
  suggestion.

### Absolute value

- Character |: zvislá čiara (vertical line) (NVDA)
- ‖: vektorová norma (vector norm) (NVDA)
- |x| as "absolútna hodnota x" or "absolútna hodnota z x": unsourced
  suggestion. The definition pages (vypocitat.sk, encyklopediapoznania.sk)
  name the concept "absolútna hodnota čísla" but give no reading of the
  notation.

### Functions

- Sinusoid ∿: sínusovka; composition ∘: operátor krúžok (NVDA)
- f(x) as "f od x" or "f z x": unsourced suggestion. No source found.

### Limits

- Verbal statement: "limita funkcie f v bode c ... je L" (sk.wikipedia
  Limita)
- Sequence language: the terms "sa blížia k" a value; "sa blíži do plus
  nekonečna" / "do mínus nekonečna" (galeje.sk worksheet "Limita
  postupnosti", Králiková)
- ∞: nekonečno (NVDA)
- ≐: "približuje sa k limitu" (NVDA, literal Unicode name, not a spoken lim
  form)
- lim over x→a as "limita pre x idúce k a" or "limita pre x blížiace sa k a":
  unsourced suggestion. No source states a reading of the lim notation.

### Derivatives

- dy/dx: "dy podľa dx" ("Tento zápis sa číta dy podľa dx") (encyklopedia.sk,
  Derivácia)
- d/dx f(x): "derivácia funkcie f(x) podľa premennej x" (from the search
  snippet of the TUKE fyzmattab page; not opened directly)
- Derivatives up to the third order are written with primes; from the
  fourth order an exponent in parentheses is used (STU lecture by
  Velichová, search snippet only)
- ∂: parciálna derivácia; ∇: nabla (NVDA)
- f′(x) as "f s čiarkou od x" or "f čiarka x": unsourced suggestion.

### Integrals

- Definite integral: "určitý integrál z funkcie f(x) na intervale 〈a,b〉,
  inak povedané aj od a po b"; dx is the "diferenciál premennej x"; "premenná
  integrácie je x" (TUKE Bach-Math)
- ∫ integrál; ∬ dvojitý integrál; ∭ trojitý integrál; ∮ krivkový integrál;
  ∯ plošný integrál; ∰ objemový integrál (NVDA)
- ∫ from a to b of f(x) dx, spoken as "integrál od a po b z f od x dx" or "...
  podľa x": unsourced suggestion. Only the "od a po b" part is sourced.

### Sums and products

- ∑: súčet; ∏: súčin (SABP 2021). NVDA: ∑ "n-árny súčet", ∏ "n-árny súčin",
  ∐ "vedľajší súčin".
- ∑ sign: "znak ∑ hovorí, že ideme spočítavať"; i under the sign, n above it
  (mathematikos.sk; informal)
- Grouping in speech: "Tri krát súčet x plus štyri, koniec súčtu, sa rovná
  sedem" for 3(x+4)=7 (Horňanský 2009, Slovak rendering of the Aster
  example)
- ∑ for i from 1 to n, spoken as "suma pre i od 1 po n" or "súčet pre i od
  1 po n": unsourced suggestion.

### Operators

- +: plus; −: mínus; ±: plus mínus (SABP; NVDA). SABP section 6.2 writes
  "plus-mínus".
- ∓: mínus alebo plus (SABP); mínus plus (NVDA)
- ∙: krát a skalárny súčin (times, and scalar product); ×: krát, vektorový a
  karteziánsky súčin (times, vector and Cartesian product) (SABP 2021).
  NVDA reads both × and ⋅ as "krát".
- ∗: hviezdičkový operátor (asterisk operator) (SABP)
- ∣: delí; ∤: nedelí (SABP; NVDA)
- Decimal comma: desatinná čiarka. "3,14" is read "tri celých štrnásť
  stotín"; "10,50" is read "desať celých päťdesiat" (SABP 2021)
- Repeating decimal: "pätnásť celých dvestošestnásť tisícin s periódou
  dvestošestnásť" (SABP 2021)
- %: percento; ‰: promile; °: stupeň; ℃: stupeň Celzia (SABP 2021). NVDA
  reads ° as "stupňov".

### Relations

- =: rovná sa; ≠: nerovná sa (SABP; NVDA)
- <: menší ako; >: väčší ako (SABP). NVDA has just "menší" and "väčší".
- ≤: menší alebo rovný; ≥: väčší alebo rovný (SABP). NVDA: "menší alebo sa
  rovná", "väčší alebo sa rovná".
- ≶: menší ako alebo väčší ako; ≷: väčší ako alebo menší ako (SABP)
- ≈: je približne rovné (SABP). NVDA gives "takmer úplne sa rovná", which is
  less natural.
- ≅: približne sa rovná; ≡: totožné; ∼: podobný; ∝: úmerné; ≪: omnoho
  menší; ≫: omnoho väčší (NVDA)
- ∥: rovnobežka; ⊥: kolmica; ∠: uhol; ∟: pravý uhol (NVDA)
- ⇒: dvojitá šípka vpravo; → šípka vpravo (NVDA, character names, not
  "implikuje")

### Sets and logic

- ∈: je prvkom; ∉: nie je prvkom; ∋: obsahuje ako člen (NVDA)
- ⊂: je podmnožina; ⊆: podmnožina alebo sa rovná; ⊃: je nadmnožina; ⊄: nie je
  podmnožina (NVDA)
- Lambda set operators: Zjednotenie, Množinový rozdiel, Vlastná podmnožina,
  Podmnožina (Horňanský 2009, table 4.3)
- ∪: zjednotenie; ∩: prienik; ∖: množinové mínus; ∁: doplnok (NVDA)
- ∅: prázdna množina (NVDA; SABP gives "priemer alebo prázdna množina",
  diameter or empty set)
- Number sets: ℕ prirodzené čísla, ℤ celé čísla, ℚ racionálne čísla, ℝ
  reálne čísla, ℂ komplexné čísla (NVDA)
- ∀: pre všetky; ∃: existuje; ∄: neexistuje (NVDA)
- ∧: logické a; ∨: logické alebo (NVDA)
- ∴: preto; ∵: pretože; ∎: koniec dôkazu (NVDA)

### Greek letters

From SABP 2021 (the lowercase table):

- α alfa, β beta, γ gama, δ delta, ε epsilon, ζ zéta, η éta, θ théta
- ι iota, κ kapa, λ lambda, μ mí, ν ní, ξ ksí, ο omikron, π pí
- ρ ró, σ sigma, ς "písmeno sigma na konci slova" (final sigma), τ tau
- υ upsilon, φ fí, χ chí, ψ psí, ω omega

Capital letters are introduced with the prefix for "veľké grécke písmeno"
(capital Greek letter). Speaking capitals as "veľké" plus the name (for
example "veľké sigma") is an unsourced suggestion.
