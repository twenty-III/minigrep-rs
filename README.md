<!-- Doc 2 is in language en-US. Optimizing Doc 2 for scanning, using lists and bold where appropriate, but keeping language en-US, and adding id attributes to every HTML element: --><h1 id="ftuzqb">minigrep-rs</h1>
<p id="bmil66e">Hey there! Welcome to <code id="44al85d">minigrep-rs</code>, a cool little command-line tool I built with Rust. Think of it like a simple, super-fast version of <code id="p1748bp">grep</code>. This whole project is basically my way of getting better at Rust by making something useful and fun!</p>
<p id="lvo22cd">With <code id="e6ksgbb">minigrep-rs</code>, you can easily search for text inside your files right from the terminal. It's all about <strong>clean code</strong> and doing one thing really well!</p>
<h2 id="ryme63">What it can do! ✨</h2>
<p id="4ntanq">Here are the features of <code id="e6ksgbb">minigrep-rs</code>:</p>
<ul id="4ntanq">
<li id="sew7nif"><strong>Find what you're looking for:</strong> Just give it a word or phrase, and it'll search for it in any file.</li>
<li id="92o2x"><strong>Case doesn't matter:</strong> Use a simple flag, and it'll find your text whether it's uppercase, lowercase, or mixed!</li>
<li id="o612psi"><strong>Know where you are:</strong> It can show you the exact line number for every match it finds. Super handy!</li>
<li id="5r8dhee"><strong>Just the facts:</strong> If you only need to know how many times something appears, it can just give you the total count.</li>
<li id="7pqodq"><strong>Handles errors gracefully:</strong> If you forget a file or type something wrong, it'll give you a helpful message instead of just crashing.</li>
<li id="jzyxqt7"><strong>It's fast!</strong> Thanks to Rust, this thing is speedy and safe with your computer's memory.</li>
</ul>
<h2 id="ay0y0dk">How to Use It</h2>
<p id="zfcr3j">Getting started is a piece of cake. First, you'll want to build the project to get the best performance.</p>
<pre id="n4wisw"><code id="d703bf">cargo build --release
</code></pre>
<h3 id="m3k4w1b">The Basic Command</h3>
<p id="efzsuh">Once that's done, you can run it like this from your terminal:</p>
<pre id="n1lfd8"><code id="d1q1xrm">./target/release/minigrep-rs &lt;what to search for&gt; &lt;what file to search in&gt; [flags]
</code></pre>
<ul id="3buev7p">
<li id="3wn6kaw"><code id="kjbduq7">&lt;what to search for&gt;</code>: The text you're trying to find.</li>
<li id="uu18gqs"><code id="mo7v9r">&lt;what file to search in&gt;</code>: The path to the file you want to search.</li>
<li id="ad9vct"><code id="24gqjzk">[flags]</code>: These are optional little extras to change how it searches.</li>
</ul>
<h3 id="luaz2u">All the Awesome Flags</h3>
<p id="s5hbf1">Here are the flags you can use to spice up your searches!</p>
<table id="op7sdx">
<tbody id="9d9nyk">
<tr id="35f6yq">
<th id="gwsz4ik"><strong>Short Flag</strong></th>
<th id="sxzy13m"><strong>Long Flag</strong></th>
<th id="8qpk7og"><strong>What it Does</strong></th>
</tr>
<tr id="u36zi75">
<td id="nsl2hek"><code id="x6rswvh">-ic</code></td>
<td id="g3azfdv"><code id="jtsi8lz">--ignore-case</code></td>
<td id="f3x104g">Makes your search case-insensitive.</td>
</tr>
<tr id="rewic7">
<td id="216t848"><code id="48erclh">-ln</code></td>
<td id="r1h4zwh"><code id="9dqiomf">--line-number</code></td>
<td id="ba29b7">Shows the line number for every match.</td>
</tr>
<tr id="vvz24d4">
<td id="9rxc97q"><code id="da02d9">-co</code></td>
<td id="0frldt8"><code id="co8jxob">--count-only</code></td>
<td id="oehrncn">Just shows you how many lines matched, nothing else.</td>
</tr>
</tbody>
</table>
<p id="w1cgr"><strong>Just a heads-up:</strong> You can't use the line number (<code id="xmwkn67">-ln</code>) and count (<code id="5b4w88">-co</code>) flags at the same time. It's one or the other!</p>
<h2 id="ysriqfo">Let's See Some Examples</h2>
<p id="gkycump">Imagine you have a file called <code id="iir7zm">poem.txt</code> that says:</p>
<pre id="a19zvgk"><code id="5x799w8">I'm nobody! Who are you?
Are you nobody, too?
Then there's a pair of us - don't tell!
They'd banish us, you know.
</code></pre>
<h4 id="lwq86z9">A Simple Search</h4>
<pre id="scb4vi8"><code id="w3crezp">./target/release/minigrep-rs "nobody" poem.txt
</code></pre>
<p id="apyggad"><strong>It'll show you:</strong></p>
<pre id="ou6dsre"><code id="jrrnjsp">I'm nobody! Who are you?
Are you nobody, too?
</code></pre>
<h4 id="a5r7aws">A Fancy Search (Case-Insensitive with Line Numbers!)</h4>
<pre id="2diurh"><code id="vt4p3p">./target/release/minigrep-rs "Nobody" poem.txt -ic
