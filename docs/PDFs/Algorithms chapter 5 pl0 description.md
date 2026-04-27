lucid, systematic, and penetrating treatment of basic and dynamic data structures, sorting, recursive algorithms, language structures, and compiling

NIKLAUS WIRTH

# Algorithms+ Data Structures= Programs

PRENTICE-HALL

SERIES IN

AUTOMATIC

COMPUTATION

# 5 LANGUAGE STRUCTURES AND COMPILERS

In this chapter we are aiming at developing a compiler (translator) for a simple, rudimentary programming language. This compiler program may serve as an example for the systematic, well-structured development of a program of non-trivial complexity and size. In this respect, it constitutes a welcome application of the program and data structuring disciplines exposed and elaborated in the preceding chapters, but in addition to this, the aim is to present a general introduction to the structure and operation of compilers. Knowledge and insight on this subject will both enhance the general understanding of the art of programming in terms of high-level languages and will make it easier for a programmer to develop his own systems appropriate for specific purposes and areas of application. Since it is well recognized that the discipline of compiler engineering is a complicated and wide subject, the chapter's character in this latter respect will necessarily be introductory and expository. Perhaps the most important single point is that the structure of language is mirrored in the structure of its compiler and that its complexity—or simplicity—intimately determines the complexity of its compiler. We shall therefore start by describing language composition and will then concentrate exclusively on simple structures that lead to simple, modular translators. Language constructs of this kind of structural simplicity are, as it turns out, adequate for virtually all genuine needs arising in practical programming languages.

# 5.1. LANGUAGE DEFINITION AND STRUCTURE

Every language is based on a vocabulary. Its elements are ordinarily called words; in the realm of formal languages, however, they are called

(basic) symbols. It is characteristic of languages that some sequences of words are recognized as correct, well-formed sentences of the language and that others are said to be incorrect or ill-formed. What is it that determines whether a sequence of words is a correct sentence or not? It is the grammar, syntax, or structure of the language. In fact, we define the syntax as the set of rules or formulas which defines the set of (formally correct) sentences. More importantly, however, such a set of rules not only allows us to decide whether or not a given sequence of words is a sentence, but it also provides the sentences with a structure which is instrumental in the recognition of a sentence's meaning. Hence, it is clear that syntax and semantics (= meaning) are intimately connected. The structural definitions are therefore always to be considered as auxiliary to a higher purpose. This, however, must not prevent us from initially studying structural aspects exclusively, ignoring the issues of meaning and interpretation.

Take, for example, the sentence, "Cats sleep." The word "cats" is the subject and "sleep" is the predicate. This sentence belongs to the language that may, for instance, be defined by the following syntax.

$$
\begin{array}{l} \langle \text {s e n t e n c e} \rangle : := \langle \text {s u b j e c t} \rangle \langle \text {p r e d i c a t e} \rangle \\ \langle \text {s u b j e c t} \rangle : := \text {c a t s} | \text {d o g s} \\ \langle \text {p r e d i c a t e} \rangle : := \text {s l e e p} | \text {e a t} \\ \end{array}
$$

The meaning of these three lines is

1. A sentence is formed by a subject followed by a predicate.  
2. A subject consists of either the single word "cats" or the word "dogs."  
3. A predicate consists of either the word "sleep" or the word "eat."

The idea then is that a sentence may be derived from the start symbol  $\langle \text{sentence} \rangle$  by repeated application of replacement rules.

The formalism or notation in which these rules are written is called Backus-Naur-Form (BNF). It was first used in the definition of ALGOL 60 [5-7]. The sentential constructs ⟨sentence⟩, ⟨subject⟩, and ⟨predicate⟩ are called non-terminal symbols; the words cats, dogs, sleep, and eat are called terminal symbols, and the rules are called productions. The symbols ::= and | are called meta-symbols of the BNF notation. If, for the sake of brevity, we use single capital letters for non-terminal symbols and lower case letters for terminal symbols, then the example can be rewritten as

EXAMPLE 1

$$
\begin{array}{l} S:: := A B \\ A := x \mid y \tag {5.1} \\ B:: = z \mid w \\ \end{array}
$$

and the language defined by this syntax consists of the four sentences  $xz$ ,  $yz$ ,  $xw$ ,  $yw$ .

To be more precise, we present the following mathematical definitions:

1. Let a language  $L = L(T, N, P, S)$  be specified by

(a) A vocabulary  $T$  of terminal symbols.  
(b) A set  $N$  of non-terminal symbols (grammatical categories).  
(c) A set  $P$  of productions (syntactical rules).  
(d) A symbol  $\bar{S}$  (from  $N$ ), called the start symbol.

2. The language  $L(T, N, P, S)$  is the set of sequences of terminal symbols  $\xi$  that can be generated from  $S$  according to rule 3 below.

$$
L = \{\xi | S \xrightarrow {*} \xi \text {a n d} \xi \in T ^ {*} \} \tag {5.2}
$$

(we use Greek letters to denote sequences of symbols.)  $T^*$  denotes the set of all sequences of symbols from  $T$ .

3. A sequence  $\sigma_{n}$  can be generated from a sequence  $\sigma_{0}$  if and only if there exist sequences  $\sigma_{1},\sigma_{2},\ldots ,\sigma_{n - 1}$  such that every  $\sigma_{i}$  can be directly generated from  $\sigma_{i - 1}$  according to rule 4 below:

$$
\left(\sigma_ {0} \xrightarrow {*} \sigma_ {n}\right) \longleftrightarrow \left(\left(\sigma_ {i - 1} \longrightarrow \sigma_ {i}\right) \quad \text {f o r} i = 1 \dots n\right) \tag {5.3}
$$

4. A sequence  $\eta$  can be directly generated from a sequence  $\xi$  if and only if there exist sequences  $\alpha, \beta, \xi', \eta'$  such that

(a)  $\xi = \alpha \xi^{\prime}\beta$  
(b)  $\eta = \alpha \eta^{\prime}\beta$  
(c)  $P$  contains the production  $\xi^{\prime}::= \eta^{\prime}$

Note: We use  $\alpha ::= \beta_1 | \beta_2 | \ldots | \beta_n$  as a short form for the set of productions  $\alpha ::= \beta_1, \alpha ::= \beta_2, \ldots, \alpha ::= \beta_n$ .

For instance, the sequence  $xz$  of Example 1 can be generated by the following sequence of direct generating steps:  $S \longrightarrow AB \longrightarrowxB \longrightarrow xz$ ; hence  $S \stackrel{*}{\longrightarrow} xz$ , and since  $xz \in T^{*}$ ,  $xz$  is a sentence of the language, i.e.,  $xz \in L$ . Note that the non-terminal symbols  $A$  and  $B$  occur in non-terminating steps only, whereas the terminating step must lead to a sequence that contains terminal symbols only. The grammatical rules are called productions because they determine how new forms may be generated or produced.

A language is said to be context free if and only if it can be defined in terms of a context-free production set. A set of productions is context free if and only if all its members have the form

$$
A := \xi \quad (A \in N, \quad \xi \in (N \cup T) ^ {*})
$$

i.e., if the left side consists of a single non-terminal symbol and can be replaced by  $\xi$  regardless of the context in which  $A$  occurs. If a production has the form

$$
\alpha A \beta : := \alpha \xi \beta ,
$$

then it is said to be context sensitive because the replacement of  $A$  by  $\xi$  may

take place only in the context of  $\alpha$  and  $\beta$ . We shall subsequently restrict our attention to context free systems.

Example 2 shows how through recursion an infinity of sentences can be generated by a finite set of productions.

EXAMPLE 2

$$
S := x A \tag {5.4}
$$

$$
A := z \mid y A
$$

The following sentences can be generated from the start symbol  $S$ .

XZ

xyz

xyyz

xyyyz

··

# 5.2. SENTENCE ANALYSIS

The task of language translators or processors is primarily not the generation but the recognition of sentences and sentence structure. This implies that the generating steps which lead to a sentence must be reconstructed upon reading the sentence, and that its generation steps must be retraced. This is generally a very complicated and sometimes even impossible task. Its complexity intimately depends on the kind of production rules used to define the language. It is the task of the theory of syntax analysis to develop recognizing algorithms for languages with rather complicated structural rules. Here, however, our goal is to outline a method for constructing recognizers that are sufficiently simple and efficient to serve in practice. This implies nothing less than that the computational effort to analyze a sentence must be a linear function of the length of the sentence; in the very worst case the dependency function may be  $n \cdot \log n$ , where  $n$  is the sentence length. Clearly, we cannot be bothered with the problem of finding a recognition algorithm for any given language, but we will work pragmatically in the reverse direction: define an efficient algorithm and then determine the class of languages that can be treated by it [5-3].

A first consequence of the basic efficiency requirement is that the choice of every analysis step must depend only on the present state of computation and on a single next symbol being read. Another most important requirement is that no step will have to be revoked later on. These two requirements are commonly known under the technical term one-symbol-lookahead without backtracking.

The basic method to be explained here is called top-down parsing because it consists of trying to reconstruct the generating steps (which in general form a structural tree) from their start symbol to the final sentence, from the

top down [5-5 and 5-6]. Let us start by revisiting Example 1: We are given the sentence, Dogs eat, and we must determine whether or not it belongs to the language. This, by definition, is only the case if it can be generated from the start symbol <sentence>. From the grammatical rules it is evident that it can only be a sentence if it is a subject followed by a predicate. We now divide the remaining task; first, we determine whether or not some initial part of the sentence may be generated from the symbol <subject>. This is indeed so since dogs can be directly generated; the symbol dogs is checked off in the input sentence (i.e., we advance our reading position), and we proceed to the second task: checking whether or not the remaining part can be generated from the symbol <predicate>. Since this is again the case, the result of the analysis process is affirmative. We may visualize this process by the following trace, showing on the left the tasks still ahead and on the right the part of the input still remaining unread.

![](images/51f7787f07a3971c4ed7c32fb397fd9bd9da41cb428c2c3212db3a48b25f4e28.jpg)

A second example shows the trace of the analysis process of the sentence xyyz according to the productions of Example 2.

<table><tr><td>S</td><td>xyyz</td></tr><tr><td>xA</td><td>xyyyz</td></tr><tr><td>A</td><td>yyz</td></tr><tr><td>yA</td><td>yyz</td></tr><tr><td>A</td><td>yz</td></tr><tr><td>yA</td><td>yz</td></tr><tr><td>A</td><td>z</td></tr><tr><td>z</td><td>z</td></tr><tr><td>-</td><td>-</td></tr></table>

Since the process of retracing the generating steps of a sentence is called parsing, what is described above is a parsing algorithm. In the two examples the individual replacement steps could decidedly be taken upon inspection of the single next symbol in the input sequence. Unfortunately, this is not always possible, as is evident from the following example:

# EXAMPLE 3

$$
\begin{array}{l} S:: := A \mid B \\ A := x A | y \tag {5.5} \\ B:: := x B | z \\ \end{array}
$$

We try to parse the sentence  $xxxz$

<table><tr><td>S</td><td>xxxz</td></tr><tr><td>A</td><td>xxxz</td></tr><tr><td>xA</td><td>xxxz</td></tr><tr><td>A</td><td>xxz</td></tr><tr><td>xA</td><td>xxz</td></tr><tr><td>A</td><td>xz</td></tr><tr><td>xA</td><td>xz</td></tr><tr><td>A</td><td>z</td></tr></table>

and get stuck. The difficulty arises in the very first step in which the decision about replacing  $S$  by  $A$  or  $B$  cannot be taken by looking at the first symbol only. A possible solution lies in simply proceeding according to one of the possible options and to retreat along the taken path of pursued subgoals if no further progress is possible. This action is called backtracking. In the language of Example 3, there is no limit to the number of steps that may have to be "undone." This situation is clearly most undesirable; hence, those language structures that lead to backtracking should be identified and avoided in practical applications. Consequently, we shall decree that we will only consider grammar systems satisfying the following restriction that specifies that the initial symbols of alternative right parts of productions be distinct.

# RULE1

Given the production

$$
A := \xi_ {1} | \xi_ {2} | \dots | \xi_ {n}
$$

the sets of initial symbols of all sentences that can be generated from the  $\xi_{i}$ 's must be disjoint, i.e.,

$$
f i r s t (\xi_ {i}) \cap f i r s t (\xi_ {j}) = \varnothing \quad \text {f o r a l l} i \neq j.
$$

The set  $\text{first}(\xi)$  is the set of all terminal symbols that can appear in the first position of sentences derived from  $\xi$ . Let this set be computed according to the following rules:

1. The first symbol of the argument is terminal:

$$
f i r s t (a \xi) = \{a \}
$$

2. The first symbol is a non-terminal symbol with the derivation rule

$$
A := \alpha_ {1} | \alpha_ {2} | \dots | \alpha_ {n}
$$

Then

$$
f i r s t (A \xi) = f i r s t (\alpha_ {1}) \cup f i r s t (\alpha_ {2}) \cup \dots \cup f i r s t (\alpha_ {n})
$$

In Example 3, we notice that  $x \in \text{first}(A)$  and  $x \in \text{first}(B)$ . Hence Rule 1 is violated by the first production. It is indeed trivial to find a syntax for the

language of Example 3 that satisfies Rule 1. The solution lies in delaying the factoring until all  $x$ 's have been dealt with. The following productions are equivalent with those of (5.5) in the sense that they generate the same set of sentences:

$$
S := C | x S \tag {5.5a}
$$

$$
C:: := y | z
$$

Unfortunately, Rule 1 is not strong enough to shield us from further trouble. Consider

EXAMPLE 4

$$
S := A x \tag {5.6}
$$

$$
A := x | \epsilon
$$

Here,  $\epsilon$  denotes the null sequence of symbols. As we try to parse the sentence  $x$ , we may proceed into the following "dead alley":

<table><tr><td>S</td><td>x</td></tr><tr><td>Ax</td><td>x</td></tr><tr><td>xx</td><td>x</td></tr><tr><td>x</td><td>—</td></tr></table>

The trouble arose because we should have followed the production  $A \coloneqq \epsilon$  instead of  $A \coloneqq x$ . This situation is called the null string problem, and it arises only for non-terminal symbols that can generate the empty sequence. In order to avoid it, we postulate

RULE 2

For every symbol  $A \in N$  which generates the empty sequence  $(A \xrightarrow{*}\epsilon)$ , the set of its initial symbols must be disjoint from the set of symbols that may follow any sequence generated from  $A$ , i.e.,

$$
f i r s t (A) \cap f o l l o w (A) = \varnothing
$$

The set  $\text{follow}(A)$  is computed by considering every production  $P_i$  of the form

$$
X:: := \xi A \eta
$$

and taking the set  $S_{i} = \text{first}(\eta_{i})$ . follow(A) is the union of all such sets  $S_{i}$ . If at least one  $\eta_{i}$  is capable of generating the empty sequence, then the set follow(X) has to be included in follow(A) as well. In Example 4, Rule 2 is violated for the symbol A since

$$
f i r s t (A) = f o l l o w (A) = \{x \}
$$

The usual way of expressing a repeated pattern of symbols is by using a recursive definition of a sentential construct. For example, the production

$$
A := B \mid A B
$$

describes the set of sequences  $B, BB, BBB, \ldots$ . Its use, however, is now

prohibited by Rule 1 because

$$
f i r s t (B) \cap f i r s t (A B) = f i r s t (B) \neq \varnothing
$$

If we replace the production by the slightly modified version

$$
A := \epsilon | A B
$$

generating the sequences  $\epsilon, B, BB, BBB, \ldots$ , we violate Rule 2 because

$$
f i r s t (A) = f i r s t (B)
$$

and therefore

$$
f i r s t (A) \cap f o l l o w (A) \neq \varnothing
$$

The two restrictive rules obviously prohibit the use of left recursive definitions. A simple method used to avoid these forms is by either using right recursion

$$
A := \epsilon | B A
$$

or by extending the symbolism of BNF to allow to express replication explicitly; we shall do so by letting  $\{B\}$  denote the set of sequences

$$
\epsilon , B, B B, B B B, \dots
$$

Of course, one must be aware that every such construct is capable of generating the empty sequence. (The brackets  $\{$  and  $\}$  are meta-symbols of extended BNF.)

From the preceding argument and from the transformation of the productions (5.5) into (5.5a) it may appear that the "trick" of transforming grammars might be the panacea to all problems of syntax analysis. We must, however, keep in mind that sentential structure is instrumental in defining sentential meaning, that explanations of the meaning of a sentential construct are usually expressed in terms of the meaning of the sentential components. Take, for example, the language of expressions consisting of operands  $a$ ,  $b$ ,  $c$  and the minus sign meaning subtraction.

$$
S:: := A \mid S - A
$$

$$
A := a \mid b \mid c
$$

According to this grammar, the sentence  $a - b - c$  has a structure that can be expressed by using parentheses as follows:  $((a - b) - c)$ . However, if the grammar is transformed into the syntactically equivalent but left recursion free form

$$
S:: := A \mid A - S
$$

$$
A := a \mid b \mid c
$$

then the same sentence would be given another structure, namely, the one expressed as  $(a - (b - c))$ . Considering the conventional meaning of subtraction, we see that the two forms are not at all semantically equivalent.

The lesson, then, is that when defining a language with an inherent meaning, one must always be aware of the semantic structure when devising its syntactic structure because the latter must reflect the former.

# 5.3. CONSTRUCTING A SYNTAX GRAPH

In the previous paragraph, a top-down recognition algorithm was presented that is applicable to grammars satisfying the restrictive Rules 1 and 2. We now turn to the problem of converting this algorithm into a concrete program. There are two essentially different techniques that can be applied. One is to design a general top-down parsing program valid for all possible grammars (satisfying Rules 1 and 2). In this case, particular grammars are to be supplied in the form of some data structure, on the basis of which the program operates. This general parser is in some sense controlled by the data structure; the program is then called table driven. The other technique is to develop a top-down parsing program which is specific for the given language and to construct it systematically according to a set of rules which map a given syntax into a sequence of statements, i.e., into a program. Both techniques have their advantages and disadvantages; both will be introduced subsequently. In the development of a compiler for a given programming language the high degree of flexibility and parametrization of the general parser are hardly needed, whereas the specific parser approach usually leads to more efficient and more easily manageable systems and is therefore preferable. In both cases it is advantageous to represent the given syntax by a so-called recognition- or syntaxgraph. This graph reflects the flow of control during the process of parsing a sentence.

It is a characteristic of the top-down approach that the goal of the parsing process is known at the start. The goal is to recognize a sentence, i.e., a sequence of symbols generatable from the start symbol. The application of a production, that is, the replacement of a single symbol by a sequence of symbols, corresponds to the splitting up of a single goal into a number of subgoals to be pursued in specified order. The top-down method is therefore also called goal-oriented parsing. In constructing a parser it is easy to take advantage of this obvious correspondence of non-terminal symbols and goals: we construct a subparser for each non-terminal symbol. Each subparser has the goal of recognizing a subsentence generatable from its corresponding non-terminal symbol. Since we wish to construct a graph to represent the total parser, each non-terminal will be mapped into a subgraph. This leads us to the following rules for constructing a recognizer graph.

RULES OF GRAPH CONSTRUCTION:

A1. Each nonterminal symbol  $A$  with corresponding production set

$$
A := \xi_ {1} | \xi_ {2} | \dots | \xi_ {n}
$$

is mapped into a recognition graph  $A$ , whose structure is determined by the righthand side of the production according to Rules A2 through A6.

A2. Every occurrence of a terminal symbol  $x$  in a  $\xi_{i}$  corresponds to a recognizing statement for this symbol and the advancing of the reader to the next symbol of the input sentence. This is represented in the graph by an edge labelled  $x$  enclosed in a circle.

![](images/7a728b43696c2eaca2c76c7641ca9528639323e910b190fd551e73d554038ddf.jpg)

A3. Every occurrence of a non-terminal symbol  $B$  in a  $\xi_{i}$  corresponds to an activation of the recognizer  $B$ . This is represented in the graph by an edge labelled  $B$ :

![](images/62af03dd55ea4cf9dfcbe237d00d64fa1f9d37eb104e0f1ee2af12195bb55b83.jpg)

A4. A production having the form

$$
A := \xi_ {1} | \dots | \xi_ {n}
$$

is mapped into the graph

![](images/8885aa8ce653e340b71a5bc2ad71af5fce700861bc64a463558f81f2595f0574.jpg)

where every  $\xi_{i}$  is obtained by applying construction Rules A2 through A6 to  $\xi_{i}$ .

A5. A  $\xi$  having the form

$$
\xi = \alpha_ {1} \alpha_ {2} \dots \alpha_ {m}
$$

is mapped into the graph

![](images/781787eb3dcce4ad6628ea89013b80f2f6a78f243f2ff35676ee81faed3a3797.jpg)

where every  $\boxed{\alpha_{i}}$  is obtained by applying construction Rules A2 through A6 to  $\alpha_{i}$ .

A6. A  $\xi$  having the form

$$
\boldsymbol {\xi} = \{\alpha \}
$$

is mapped into the graph

![](images/954cd08b7b496278c24b8d3320e823e32fed31ca7ef5b7488ce3352256d2d358.jpg)

where  $\boxed{\alpha}$  is obtained by applying construction Rules A2 through A6 to  $\alpha$ .

EXAMPLE 5

$$
\begin{array}{l} A := x | (B) \\ B \because = A C \tag {5.7} \\ C:: := \{+ A \} \\ \end{array}
$$

Here,  $+, x, ($ , and) are the terminal symbols, whereas  $\{$  and  $\}$  belong to the extended BNF and, hence, are meta-symbols. The language generatable from  $A$  consists of expressions with operands  $x$ , operator  $+$ , and parentheses. Examples of sentences are

x

$$
(x)
$$

$$
\begin{array}{l} (x + x) \\ ((x)) \\ \end{array}
$$

···

The graphs resulting from the application of the six construction rules are shown in Fig. 5.1. Note that it is possible to reduce this system of graphs into a single graph by suitable substitution of  $C$  in  $B$  and of  $B$  in  $A$  (see Fig. 5.2).

![](images/c59a4ac5d8254b3909a8ea58f8a6ca4e97dc926ee6843b03b288ec0ccd400dc0.jpg)  
Fig. 5.1 Syntax graphs according to Example 5.

![](images/478a99d0bad1c5b291229d4512efebec32e3150aa69a67e7823ab1aed4a0721d.jpg)  
Fig. 5.2 Reduced syntax graph corresponding to Example 5.

The recognition graph is an equivalent representation of the language grammar; it can be used instead of the set of productions in BNF. It is a very convenient form and in many (if not most) instances preferable to BNF. It certainly gives a clearer and more concise picture of a language structure and also conveys a more direct understanding of the parsing process. The graph is an appropriate form for the designer of a language to start from. Examples of syntax specifications of entire languages are shown in Sect. 5.7 for PL/0, and in Appendix B for PASCAL.

Restrictive Rules 1 and 2 were imposed in order to allow for deterministic parsing with only one symbol lookahead. How are these rules manifested in the graph representation? It is in this respect that the clarity of the graph becomes most obvious:

1. Rule 1 translates into the requirement that at every fork the branch to be pursued must be selectable by looking only at the next symbol on this branch. This implies that no two branches must start with the same next symbol.  
2. Rule 2 translates into the requirement that if any graph  $A$  can be traversed without reading an input symbol at all, then this "null branch" must be labelled with all symbols that may follow  $A$ . (This will affect the decision to be made upon entering this branch).

It is simple to verify, whether or not a system of graphs satisfies these two adapted rules, without resorting to a BNF representation of the grammar. As an auxiliary step, the sets first(A) and follow(A) are determined for each graph  $A$ . Application of Rules 1 and 2 is then immediate. We call a system of graphs that satisfies these two rules a deterministic syntax graph.

# 5.4. CONSTRUCTING A Parser FOR A GIVEN SYNTAX

A program which accepts and parses a language is readily derived from its deterministic syntax graph (if such a graph exists). The graph essentially represents the flowchart of the program. In developing this program, how-

ever, one is well-advised to strictly follow a given set of translation rules similar to those that may have led from a BNF to a graph representation of the syntax in the first place. These rules are listed below. They are applicable in a specific framework. This framework consists of a main program in which the procedures corresponding to the various subgoals are embedded and of a routine to proceed to the next symbol.

For the sake of simplicity, let us assume that the sentence to be parsed is represented by the file input and that terminal symbols are individual characters. We now postulate the existence within this framework of a character variable  $ch$  that always represents the next symbol being read. Stepping to the next symbol is then expressed by the statement

# read(ch)

The main program now consists of an initial statement to read the first character, followed by a statement activating the main parsing goal. The individual routines corresponding to parsing goals or graphs are obtained by obeying the following rules. Let the statement obtained by translating the graph  $S$  be denoted by  $T(S)$ .

RULES OF GRAPH TO PROGRAM TRANSLATION:

B1. Reduce the system of graphs to as few individual graphs as possible by appropriate substitutions.  
B2. Translate each graph into a procedure declaration according to the subsequent rules B3 through B7.  
B3. A sequence of elements

![](images/514cd9da090a8b3b064a5bbc78dc1e4d42992963a7005cc7a70185012e0ae4ea.jpg)

is translated into the compound statement

begin  $T(S_{1})$  .  $T(S_{2})$  . . .;  $T(S_{n})$  end

B4. A choice of elements

![](images/6dbc9971d8a1d5fb5459a3cf771d253a2b66751907040a24b22b447bada2455f.jpg)

is translated into the selective or conditional statement

$$
\begin{array}{c} \hline \text {c a s e c h o f} \\ L _ {1}: T (S _ {1}); \\ L _ {2}: T (S _ {2}); \\ \dots \dots \\ L _ {n}: T (S _ {n}) \\ \text {e n d} \end{array}
$$

if ch in  $L_{1}$  then  $T(S_{1})$  else if ch in  $L_{2}$  then  $T(S_{2})$  else  $\dots \dots \dots$  if ch in  $L_{n}$  then  $T(S_{n})$  else error

where  $L_{i}$  denotes the set of initial symbols of the construct  $S_{i}(L_{i} = \text{first}(S_{i}))$ . Note: if  $L_{i}$  consists of a single symbol  $a$ , then of course "ch in  $L_{i}$ " should be expressed as "ch = a".

B5. A loop of the form

![](images/07f2e0f4e86d62f1b7b3cee5fe3246e61f2cbc55c94fe7052654092f9cab8322.jpg)

is translated into the statement

while ch in  $L$  do  $T(S)$

where  $T(S)$  is the translation of  $S$  according to rules B3 through B7, and  $L$  is the set  $L = \text{first}(S)$  (see preceding note).

B6. An element of the graph denoting another graph  $A$

![](images/e8c5d936f80a253ba9e3d59060cd250d667ed88944214536931349e56d397226.jpg)

is translated into the procedure call statement  $A$ .

B7. An element of the graph denoting a terminal symbol  $x$

![](images/460ffb1c9b3e54cca970424202a399a31d6fdcf6d12051090ed165c6f9577c02.jpg)

is translated into the statement

if  $ch = x$  then read(ch) else error

where error is a routine called when an ill-formed construct is encountered.

The application of these rules is now demonstrated by translating the reduced graph of Example 5 (Fig. 5.2) into a recognizer program (Program 5.1).

Program 5.1 Parsing Program for Grammar of Example 5.  
```txt
program parse (input, output);  
var ch: char;  
procedure A;  
begin if ch = 'x' then read(ch) else  
if ch = '(' then  
begin read(ch); A;  
while ch = '+' do  
begin read(ch); A  
end;  
if ch = ')' then read(ch) else error  
end else error  
end;  
begin read(ch); A  
end
```

During this translation some obvious programming rules have been freely applied in order to simplify the program. A literal translation would have resulted, for example, in the fourth line reading

```txt
if  $ch = x'$  then if  $ch = x'$  then read(ch) else error else.
```

which can obviously be reduced into the simpler form presented in the program. Also the read statements in the fifth and seventh lines resulted from similar reductions.

It seems sensible to find out where such reductions are possible in general and then to represent them directly in terms of the graph. The two relevant cases are covered by the following additional rules:

B4a

![](images/5bf7c70302ba6476710d11a0166cd0950cf24abba4b0a8c0c159b95dafe37f69.jpg)

if  $ch = 'x_1'$  then begin read(ch);  $T(S_1)$  end else

if  $ch = x_2'$  then begin read(ch);  $T(S_2)$  end else

.

if  $ch = x_{n}^{\prime}$  then begin read(ch);  $T(S_{n})$  end else error

B5a

![](images/102e5785eef081e4023a38cb513a79e5e5d631c21b607757a075b09b6e1b9a38.jpg)

while  $ch = 'x'$  do

begin read(ch);  $T(S)$  end

In addition, the frequently occurring construct

read(ch);  $T(S)$

while  $B$  do

begin read(ch);  $T(S)$  end

can of course be expressed by the shorter form

$$
\text {r e p e a t} \quad \operatorname {r e a d} (c h); T (S) \text {u n t i l} B \tag {5.8}
$$

The procedure error has so far been left unspecified on purpose. Since we are now only interested in finding out whether an input sequence is well- or ill-formed, we may think of this procedure as a program terminator. Naturally, in practice, more refined principles of coping with ill-formed sentences have to be used. This will be the subject of Sect. 5.9.

# 5.5. CONSTRUCTING A TABLE-DRIVEN PARSING PROGRAM

Instead of composing a specific program according to the rules given in the preceding chapter for each language and syntax that arises, one may construct a single, general parsing program. Individual language grammars are then fed to the general program in the form of initial data preceding the sentences that are to be parsed. The general program strictly follows the rules of the simple top-down parsing method, and it is straightforward if the underlying syntax graph is deterministic, that is, if the grammar is such that sentences can be parsed with one symbol of lookahead and without backtracking.

Hence, the grammar, which we assume to be represented in the form of a deterministic set of syntax graphs, is translated into an appropriate data structure instead of into a program structure [5-2]. The natural technique

for representing a graph is by introducing a node for each symbol and by connecting these nodes by pointers. Hence, the "table" is not a simple array structure. The rules which guide the translation are given below and are self-evident. The nodes of the data structure are records of two variants: one for terminal and the other for non-terminal symbols. The former are identified by the terminal symbol for which they stand, the latter by a pointer to the data structure representing the corresponding non-terminal symbol. Both variants contain two pointers, one designating the symbol that follows, the successor, and the other forming the list of possible alternatives. The resulting data type definition is given in (5.9), and within graphs we will depict a node as

<table><tr><td></td><td></td></tr><tr><td>alt</td><td>suc</td></tr></table>

At it turns out, we also need an element to represent the empty sequence, the null symbol. We shall denote it by a terminal element, called empty.

$$
\begin{array}{l} \text {t y p e p o i n t e r} = \uparrow \text {n o d e}; \\ \text {n o d e} = \\ \text {r e c o r d s u c , a l t : p o i n t e r ;} \\ \text {c a s e t e r m a l : b o o l e a n o f} \\ \text {t r u e : (t s y m : c h a r) ;} \\ \text {f a l s e : (n s y m : h p o i n t e r)} \\ \text {e n d} \end{array} \tag {5.9}
$$

The translation rules from graphs into data structures are analogous to Rules B1 through B7.

RULES OF GRAPH TO DATA STRUCTURE TRANSLATION:

C1. Reduce the system of graphs to as few individual graphs as possible by suitable substitution.  
C2. Translate each graph into a data structure according to the subsequent rules C3 through C5.  
C3. A sequence of elements (see picture of Rule B3) is translated into the following list of data nodes:

![](images/6ac0bb80384f258ce49ff96212e833e531bbe66d1ac3b8cc2470c23c40e2593e.jpg)

C4. The list of alternatives (see picture of Rule B4) is translated into the data structure

![](images/7fe76e657aed22d59e0f070aca6c21393a0a96518dd2826cbbe28f6f51d6f3ab.jpg)

C5. A loop (see picture in Rule B5) is translated into the structure

![](images/fdac8d1d0c959e6c8c443e8e969039326b197746cad3373ccf9a2591cad7b07c.jpg)

As an example, the graph corresponding to the syntax of Example 5 (Fig. 5.2) results in the structure in Fig. 5.3. The data structure is identified by a header node that contains the name of the non-terminal symbol (goal) for which the structure stands. This header is so far unnecessary, for the

![](images/8ae7a87a037f1be645d6d448ea8117d5c5efeacce712080777f909809d2efb98.jpg)  
Fig. 5.3 Data structure representing graph of Fig. 5.2.

pointer of the goal field could as well be pointing directly at the "entrance" of the appropriate structure. The header may be used, however, to carry a printable name of the structure.

$$
\text {t y p e} = \uparrow \text {h e a d e r};
$$

$$
h e a d e r =
$$

$$
\text {r e c o r d} \quad \text {e n t r y : p o i n t e r ;} \tag {5.10}
$$

$$
s y m: c h a r
$$

end

The program to parse a sentence—represented as a sequence of characters on the input file—now consists of a repeated statement describing the transition from one node to a next node. The program is expressed as a procedure describing the interpretation of a graph; if a node representing a non-terminal symbol is encountered, then the interpretation of that graph precedes the completion of the interpretation of the present graph. Hence, the interpretation procedure is activated recursively. If the current symbol (sym) in the input file matches the symbol in the current node of the data structure, then the suc field is selected to indicate the next step, otherwise the alt field.

procedure parse(goal: hpointer; var match: boolean);  
var s : pointer;  
begin \(s := \text{goal} \uparrow\).entry;  
repeat  
if \(s \uparrow\).terminal then  
begin if \(s \uparrow\).tsym = sym then  
begin match := true; getsym  
end  
else match := (s \uparrow\).tsym = empty)  
end  
else parse(s \uparrow\).nsym, match);  
if match then \(s := s \uparrow\).succ else \(s := s \uparrow\).alt  
until \(s = \text{nil}\)  
end

The parsing program (5.11) has the property of immediately pursuing a new subgoal  $G$  whenever one appears, without first inspecting whether or not the current symbol is contained in the set of initial symbols  $\text{first}(G)$ . This implies that the underlying syntax graph must be void of choices between several alternative non-terminal elements. In particular, if a non-terminal symbol is capable of generating the empty sequence, then none of its right parts must start with a non-terminal symbol.

More sophisticated table-driven parsers may readily be derived from (5.11) which operate on less restrictive classes of grammars. Only slight modifications will also enable it to perform backtracking with, however, notable loss of effectiveness.

Representing a syntax by a graph has one decisive disadvantage: computers cannot directly read graphs. But the data structure that drives the parser must somehow be constructed before parsing can start. It is in this respect that the BNF-representation of grammars appears ideal—as input form for general parsing programs. The next section is therefore devoted to the design of a program which reads a sequence of BNF-productions and transforms them according to rules B1 through B6 into an internal data structure, upon which the parser (5.11) can operate [5-8].

# 5.6. A TRANSLATOR FROM BNF INTO PARSER-DRIVING DATA STRUCTURES

A translator accepting BNF-productions, converting them into some other representation, is a genuine example of a program whose input data can be regarded as sentences belonging to a language. In fact, it is most appropriate to consider BNF as a language itself, characterized by its own syntax that may, of course, once again be specified in terms of BNF-productions. As a consequence, this translator may serve as a further example of the construction of a recognizer that is, moreover, extended into a translator, or, in general, a processor of its input. Therefore, we shall proceed in the following manner:

Step 1. Define a syntax of the meta language, called EBNF (for Extended BNF).  
Step 2. Construct a recognizer for EBNF according to the rules given in Sect. 5.4.  
Step 3. Extend the recognizer into a translator, combining it with the table-driven parser.

Let the meta-language—the language of syntax productions—be described by the following productions:

$$
\begin{array}{l} \langle \text {p r o d u c t i o n} \rangle \quad : := \langle \text {s y m b o l} \rangle = \langle \text {e x p r e s s i o n} \rangle . \\ \begin{array}{l l} \langle \text {e x p r e s s i o n} \rangle & \because = \langle \text {t e r m} \rangle \{\text {,} \langle \text {t e r m} \rangle \} \\ \langle \text {t e r m} \rangle & \because = \langle \text {f a c t o r} \rangle \{\langle \text {f a c t o r} \rangle \} \end{array} \tag {5.12} \\ \langle \text {f a c t o r} \rangle \quad : := \langle \text {s y m b o l} \rangle \mid [ \langle \text {t e r m} \rangle ] \\ \end{array}
$$

Note that symbols different from the usual BNF meta-symbols have been used to denote exactly these symbols in the production input language. There are two reasons for this:

1. To distinguish meta-symbols and language symbols in (5.12).  
2. To use characters more commonly available on computing equipment and, in particular, to be able to use the single character  $=$  instead of  $\vdots :=$

The correspondences of usual BNF with our input form are shown in Table 5.1. In addition, each production is terminated by an explicit period.

<table><tr><td>BNF</td><td>Input EBNF</td></tr><tr><td>::=</td><td>=</td></tr><tr><td>|</td><td>,</td></tr><tr><td>{</td><td>[</td></tr><tr><td>}</td><td>]</td></tr></table>

Table 5.1 Meta and Language Symbols.

Using this input language to describe the syntax of Example 5 (5.7), we obtain

$$
\begin{array}{l} A = x, (B). \\ B = A C. \tag {5.13} \\ C = [ + A ]. \\ \end{array}
$$

In order to simplify the translator to be constructed, we postulate that terminal symbols be single letters and that each production be written on a separate line. This includes the possibility of using blanks in the input (to make it more readable) and of ignoring these blanks by the translator. However, the statement read(ch) in Rule B7 must now be replaced by a call to a routine that obtains the next relevant character. This is a very simple case of what is generally called a lexical scanner, or simply a scanner. The purpose of a scanner is to extract the next symbol—as defined by language representation rules—from the input sequence of characters. So far we have considered symbols to be identical to characters; this, however, is a special case and is rarely done in practice.

As a last rule, in the input-BNF we postulate that non-terminal symbols be represented by the letters A through H and terminal symbols by the letters I through Z. This is merely a rule of convenience that has no deeper reasons. But it makes it unnecessary to list the vocabularies of terminal and non-terminal symbols prior to the list of productions.

By proceeding strictly according to the parser construction rules B1 through B7, and after having verified that (5.12) satisfies the Restrictive Rules 1 and 2, we obtain Program 5.2 as a recognizer for the language specified by (5.12). Note that the scanner is called getsym.

```txt
program parser(input, output);  
label 99;  
const empty = '*';  
var sym: char;  
procedure getsym;  
begin  
repeat read.sym; write.sym) until sym ≠ ';'  
end {getsym};  
procedure error;  
begin writeln;  
writeln('INCORRECT INPUT'); goto 99  
end {error};  
procedure term;  
procedure factor;  
begin  
if sym in ['A'..Z', empty] then getsym else  
if sym = '[ then  
begin getsym; term;  
if sym = '] then getsym else error  
end else error  
end {factor};  
begin factor;  
while sym in ['A'..Z', '[ empty] do factor  
end {term};  
procedure expression;  
begin term;  
while sym = ', do  
begin getsym; term  
end  
end {expression};  
begin {main program}  
while --eof(input) do  
begin getsym;  
if sym in ['A'..Z'] then getsym else error;  
if sym = ': ' then getsym else error;  
expression;  
if sym ≠ ';' then error;  
writeln; readln;  
end;  
99: end.
```

Program 5.2 Parser of Language (5.12).

Step 3 in the development of the translator is concerned with constructing the desired data structure that represents the BNF productions read and that can be interpreted by the parsing procedure (5.11). Unfortunately, this step is not susceptible to formalization as was the step concerned with the recognizer construction. Lacking a formal approach, we describe once again the structures that are desired to represent each language construct by a picture. The resulting structures are then passed as result parameters of the corresponding recognition procedures, which by then have been augmented into translation procedures. It is natural to return as results not the data structures themselves, but pointers  $p, q, r$  referring to the structures instead.

Factors: p sym 1.(symbol) q

![](images/11aa9d9ed29493955b885dbcdf4ad0c89e7ea2ff7a1b45aa65cec92ce94fbb7f.jpg)

Terms:

factor-1> factor-n

![](images/1c2b45f96e32b21218ac60308fc4a7a246264474e5abe96446c0f9bbf2db277b.jpg)

![](images/cc8ae10be48e40d20788b7958262dfb1002934af505226bf73a5b7010cacc8c3.jpg)

Expressions:

![](images/3e3da7b92a855f290f5fb0bc829c66e2820861940f7dfbdf10f7943c8f658a15.jpg)

Clearly, it is the task of the procedure factor to generate new elements of the data structure; the task of the remaining two procedures is to link them together into a linear list in which term uses the suc field and expression uses the alt field for chaining. The details are evident from Program 5.3.

The technique of processing non-terminal symbols needs further clarification. It is possible for a non-terminal symbol to appear as a factor before it appears as a left part in a production. A procedure find(sym, h) is used to locate the symbol sym in a linear list in which all headers representing the non-terminal symbols are collected. If a symbol is located, its reference is assigned to  $h$ ; if it is not yet present in the list, it is added to the list. Procedure find uses the sentinel technique discussed in detail in Chap. 4.

Program 5.3 consists of three parts, each corresponding to a section of input. Part 1 is concerned with the processing of productions into corresponding data structures. Part 2 reads and identifies a single symbol, namely, the one specified as the start symbol which generates sentences of the language. (It is preceded by a $ sign delimiting Parts 1 and 2 of the input data.) Part 3 is the parsing program (5.11) reading input sentences under control of the data structure generated in Part 1.

It is noteworthy that Program 5.3 has been developed by merely inserting further statements into the unchanged Program 5.2. The existing program deals exclusively with the recognition of correctly formed sentences, and it can be used as a framework for the extended program that not only recognizes but also processes or translates the accepted sentences. This method of constructing language processors by stepwise refinement, or rather stepwise enrichment, is highly recommended. It permits the designer to deal exclusively with a selected aspect of language processing before taking into account other aspects, and hence it facilitates the task of verifying the correctness of a translator program, or at least of maintaining a high confidence level throughout the program's development. In this rather simple example this development consists of only two steps. More complicated languages and more complicated translation tasks require a considerably higher number of individual enrichment steps. A highly similar development in three steps will be the subject of Sects. 5.8 through 5.11.

As evidenced by the development of Program 5.3, the syntax table-driven—or rather data structure-driven—approach to parsing provides a degree of freedom and flexibility not presented in the scheme of the specific parser program. This additional flexibility, although not required in general, is the very essence of compilers for so-called extensible languages. An extensible language can be extended by further syntactic constructs, more or less at the discretion of the programmer. Analogous to the input of Program 5.3, the input of an extensible language compiler consists of a section specifying the language extensions used in the subsequent program. A more ambitious

scheme even allows altering the language during the process of compilation by intermixing parts of the program to be translated with sections of new language specifications.

As appealing or exciting as these ideas may seem, efforts to realize such compilers have been marked by a notable lack of success. The reason is that the aspect of syntax and of sentence recognition is but a part of the whole task of translation, and in fact even the minor part. It is also the part that is most easily formalized and is therefore most readily represented by a systematized table structure. The much harder part to formalize is the meaning of the language, that is, the output or result of translation. This problem has so far not been solved even nearly satisfactorily, which explains why compiler designers tend to be much more enthusiastic about extensible languages before than after their first completed assignment. We conclude our lesson by devoting the remainder of this chapter to developing a modest compiler for one specific, small programming language.

Program 5.3 Translator of Language (5.12).  
```txt
program generalparser (input, output);  
label 99;  
const empty = '\*';  
type pointer = ↑node;  
hpointer = ↑header;  
node = record suc, alt: pointer;  
    case terminal: boolean of  
        true: (tsym: char);  
        false: (nsym: hpointer)  
    end ;  
    header = record sym: char;  
        entry: pointer;  
        suc: hpointer  
    end ;  
var list, sentinel, h: hpointer;  
    p: pointer;  
    sym: char;  
    ok: boolean;  
procedure getsym;  
begin  
    repeat read.sym); write.sym) until sym ≠ ';'  
end {getsym};
```

Program 5.3 (Continued)  
```vhdl
procedure find(s: char; var h: hpointer);  
{locate nonterminal symbol s in list. if not present, insert it}  
var h1: hpointer;  
begin h1 := list; sentinel↑.sym := s;  
while h1↑.sym ≠ s do h1 := h1↑.succ;  
if h1 = sentinel then  
begin {insert} new (sentinel);  
h1↑.succ := sentinel; h1↑.entry := nil  
end;  
h := h1  
end {find};  
procedure error;  
begin writeln;  
writeln ('INCORRECT SYNTAX'); goto 99  
end {error};  
procedure term (var p,q,r: pointer);  
var a,b,c: pointer;  
procedure factor (var p,q: pointer);  
var a,b: pointer; h: hpointer;  
begin if sym in ['A'..'Z', empty] then  
begin {symbol} new(a);  
if sym in ['A'..'H'] then  
begin {nonterminal} find.sym,h);  
a↑.terminal := false; a↑.nsym := h  
end else  
begin {terminal}  
a↑.terminal := true; a↑.tsym := sym  
end;  
p := a; q := a; getsym  
end else  
if sym = '[ then  
begin getsym; term(p,a,b); b↑.succ := p;  
new(b); b↑.terminal := true; b↑.tsym := empty;  
a↑.alt := b; q := b;  
if sym = ';' then getsym else error  
end else error  
end {factor};  
begin factor(p,a); q := a;  
while sym in ['A'..'Z', ';' empty] do  
begin factor(a↑.succ, b); b↑.alt := nil; a := b  
end;  
r := a  
end {term};
```

```vhdl
procedure expression (var p,q: pointer);
var a,b,c: pointer;
begin term(p,a,c): c↑.succ := nil;
while sym = ', do
begin getsym;
    term(a↑.alt, b, c): c↑.succ := nil; a := b
end;
q := a
end {expression};
procedure parse (goal: hpointer; var match: boolean);
var s: pointer;
begin s := goal↑.entry;
repeat
    if s↑.terminal then
        begin if s↑.tsym = sym then
            begin match := true; getsym
                end
            else match := (s↑.tsym = empty)
        end
        else parse(s↑.nsym, match);
    if match then s := s↑.succ else s := s↑.alt
until s = nil
end {parse};
begin {productions}
getsym; new(sentinel); list := sentinel;
while sym ≠ '$' do
begin find(sym,h);
getsym; if sym = ':=' then getsym else error;
expression (h↑.entry, p); p↑.alt := nil;
if sym ≠ ':' then error;
writeln; readln; getsym
end;
h := list; ok := true; {check whether all symbols are defined}
while h ≠ sentinel do
begin if h↑.entry = nil then
    begin writeln(UNDEFINED SYMBOL', h↑.sym);
ok := false
end;
h := h↑.succ
end;
```

Program 5.3 (Continued)

Program 5.3 (Continued)  
```txt
if -ok then goto 99;  
{goal symbol}  
getsym; find.sym,h); readln; writeln;  
{sentences}  
while --eof(input) do  
begin write(''); getsym; parse(h,ok);  
if ok ∧ (sym='.') then writeln('CORRECT')  
else writeln('INCORRECT');  
readln  
end;  
99: end.
```

# 5.7. THE PROGRAMMING LANGUAGE PL/0

The remaining sections of this chapter are devoted to the development of a compiler for a language to be called PL/0. The necessity of keeping this compiler reasonably small in order to fit into the framework of this book and the desire to be able to expose the most fundamental concepts of compiling high-level languages constitute the boundary conditions for the design of this language. There is no doubt that either an even simpler or a much more complicated language could have been chosen; PL/0 is one possible compromise between sufficient simplicity to make the exposition transparent and sufficient complexity to make the project worthwhile. A considerably more complicated language is PASCAL, whose compiler was developed using the same techniques, and whose syntax is shown in Appendix B.

As far as program structures are concerned, PL/0 is relatively complete. It features, of course, the assignment statement as the basic construct on the statement level. The structuring concepts are those of sequencing, conditional execution and repetition, represented by the familiar forms of begin/end-, if-, and while statements. PL/0 also features the subroutine concept and, hence, contains a procedure declaration and a procedure call statement.

In the realm of data types, however, PL/0 adheres to the demand for simplicity without compromise: integers are its only data type. It is possible to declare constants and variables of this type. Of course, PL/0 features the conventional arithmetic and relational operators.

The presence of procedures, that is, of more or less "self-contained" partitions of a program offers the opportunity to introduce the concept of locality of objects (constants, variables, and procedures). PL/0 therefore

features declarations in the heading of each procedure, implying that these objects are understood to be local to the procedure in which they are declared.

This brief introduction and overview provide the necessary intuition to understand the syntax of PL/0. This syntax is presented in Fig. 5.4 in the form of seven diagrams. The task of transforming the diagrams into a set of equivalent BNF-productions is left to the interested reader. Fig. 5.4 is a convincing example of the expressive power of these diagrams which allow formulation of the syntax of an entire programming language in such a concise and readable form.

The following PL/0 program may demonstrate the use of some features that are included in this mini-language. The program contains the familiar algorithms for multiplication, division, and finding the greatest common divisor (gcd) of two natural numbers.

Program

![](images/c07456e0fd54335d051d42ef37045f7672bec0d76bfd164e5a04c45c4779f04b.jpg)

Block

![](images/c745100f703f7cca1101d9162a97b537a69f2892faae11e5e2cc2822321a3f39.jpg)  
Fig. 5.4 Syntax of PL/0.

![](images/9f64d49ac51afb4ad9e14487428362144f171bf57c765307f4ca352f5042cc81.jpg)

![](images/870edf46a4a31ee0e0078758053e1f01ac24baa06b7c81d84796076775ab50d1.jpg)  
Fig. 5.4 (Continued)

Expression

![](images/aa028a0d686453a5eb4c768069a921904c4baf8c59e5a2f82c40b4228aeabc16.jpg)

Term

![](images/7abcee6ba732498f81c65bf0d8494b9671a05b0d6438c8217b28d1ae0dcf9ee4.jpg)

Factor

![](images/1323f9ae5f663333215469a6537fde71b0c34502865b3d2a9ed6b1595a858c66.jpg)  
Fig. 5.4 (Continued)

```txt
const  $m = 7$  n=85;   
var  $x,y,z,q,r$  .   
procedure multiply;   
 $\operatorname {var}a,b;$    
begin  $a\coloneqq x;b\coloneqq y;z\coloneqq 0;$  while  $b > 0$  do begin if odd  $^b$  then  $z\coloneqq z + a$ $a\coloneqq 2*a;b\coloneqq b / 2;$  end   
end ;
```

```txt
procedure divide;  
var w;  
begin  $r := x$ ;  $q := 0$ ;  $w := y$ ;  
while  $w \leq r$  do  $w := 2 * w$ ;  
while  $w > y$  do  
begin  $q := 2 * q$ ;  $w := w / 2$ ;  
if  $w \leq r$  then  
begin  $r := r - w$ ;  $q := q + 1$   
end  
end;  
procedure gcd;  
var  $f, g$ ;  
begin  $f := x$ ;  $g := y$ ;  
while  $f \neq g$  do  
begin if  $f < g$  then  $g := g - f$ ;  
if  $g < f$  then  $f := f - g$ ;  
end;  
 $z := f$   
end;  
begin  
 $x := m$ ;  $y := n$ ; call multiply;  
 $x := 25$ ;  $y := 3$ ; call divide;  
 $x := 84$ ;  $y := 36$ ; call gcd;  
end.
```

# 5.8. A PARSER FOR PL/0

As a first step toward the PL/0 compiler a parser is being developed. This can be done strictly according to the parser Construction Rules B1 through B7 outlined in Sect. 5.4. This method, however, is only applicable if the Restrictive Rules 1 and 2 are satisfied by the underlying syntax. We are therefore obliged to verify this condition, as formulated for their application to syntax graphs.

Rule 1 specifies that every branch emanating from a fork point must lead toward a distinct first symbol. This is very simple to verify on the syntax diagrams of Fig. 5.4. Rule 2 applies to all graphs that can be traversed without reading any symbol. The only such graph in the PL/0 syntax is the one describing statements. Rule 2 demands that all first symbols that may follow a statement must be disjoint from initial symbols of statements. Since later on it will be useful to know the sets of initial and following symbols for all graphs, we shall determine these sets for all seven non-terminal symbols (graphs) of the PL/0 syntax (except for "program"). Table 5.2 provides the

<table><tr><td>Non-terminal Symbol S</td><td>Initial Symbols L(S)</td><td>Follow Symbols F(S)</td></tr><tr><td>Block</td><td>const var procedure ident if call begin while</td><td>. ;</td></tr><tr><td>Statement</td><td>ident call begin if while</td><td>. ; end</td></tr><tr><td>Condition</td><td>odd + -- (ident number</td><td>then do</td></tr><tr><td>Expression</td><td>+ - (ident number</td><td>. ; ) R end then do</td></tr><tr><td>Term</td><td>ident number (</td><td>. ; ) R + - end then do</td></tr><tr><td>Factor</td><td>ident number (</td><td>. ; ) R + - * / end then do</td></tr></table>

Table 5.2 Initial and Follow Symbols in PL/0.

desired assurance, namely, that the sets of initial and following symbols of statements do not intersect. Application of the parser Construction Rules B1 through B7 is thereby legalized.

The careful reader will have noticed that the basic symbols of PL/0 are no longer single characters as in the preceding examples. Instead, the basic symbols are themselves sequences of characters, such as BEGIN, or :=. As in Program 5.3, a so-called scanner is used to take care of the merely representational or lexical aspects of the input sequence of symbols. The scanner is conceived as a procedure getsym whose task is to get the next symbol. The scanner serves the following purposes:

1. It skips separators (blanks).  
2. It recognizes reserved words, such as BEGIN, END, etc.  
3. It recognizes non-reserved words as identifiers. The actual identifier is assigned to a global variable called id.  
4. It recognizes sequences of digits as numbers. The actual value is assigned to a global variable num.  
5. It recognizes pairs of special characters, such as  $\coloneqq$

In order to scan the input sequence of characters, getsym uses a local procedure getch whose task is to get the next character. Apart from this main purpose, getch also

1. Recognizes and suppresses line end information.  
2. Copies the input onto the output file, thus generating a program listing.  
3. Prints a line number or location counter at the beginning of each line.

The scanner constitutes the necessary one-symbol lookahead. Moreover, the auxiliary procedure getch represents an additional lookahead of one

character. Therefore, the total lookahead of this compiler is one symbol plus one character.

The details of these routines are evident from Program 5.4 which represents the complete parser for PL/0. In fact, this parser is already extended in the sense that it collects the declared identifiers denoting constants, variables, and procedures in a table. The occurrence of an identifier within a statement then causes a search of this table to determine whether or not the identifier had been properly declared. The lack of such a declaration may duly be regarded as a syntactic error since it is a formal error in the composition of the program text because of the use of an "illegal" symbol. The fact that this error can only be detected by retaining information in a table is a consequence of the inherent context dependence of the language, manifest in the rule that all identifiers have to be declared in the appropriate context. Indeed, practically all programming languages are context sensitive in this sense; nevertheless, the context-free syntax is a most helpful model for these languages and greatly aids in the systematic construction of their recognizers. The framework thus obtained can then very easily be extended to take care of the few context sensitive elements of the language, as witnessed by the introduction of the identifier table in the present parser.

Before constructing the individual parser procedures corresponding to the individual syntax graphs, it is useful to determine how these graphs depend on each other. To this end, a so-called dependence diagram is constructed; it displays the relationships of the individual graphs, i.e., it lists for each graph  $G$  all those graphs  $G_{1} \ldots G_{n}$  in terms of which  $G$  is defined. Correspondingly, it shows those procedures that will be called by other procedures. The dependence graph for PL/0 is shown in Fig. 5.5.

![](images/d1bed5714fba800b3e7be7a8599f44ea98f539cf4e88587293dc8cbd11252bbd.jpg)  
Fig. 5.5 Dependence diagram for PL/0.

The loops in Fig. 5.5 indicate instances of recursion. It is therefore essential that a language in which the PL/0 compiler is implemented is not burdened by prohibition of recursion. In addition, the dependence diagram also allows drawing conclusions on the hierarchical organization of the parser program. For instance, all routines may be contained in (be declared local to) the routine that parses the construct ⟨program⟩ (which is therefore the main program part of the parser). Furthermore, all routines below ⟨block⟩ may be defined locally to the routine representing the parsing goal ⟨block⟩. Naturally, all of these routines call upon the scanner getsym, which in turn calls upon getch.

Program 5.4 PL/0 Parser.  
program PL0 (input, output);  
{PL/0 compiler, syntax analysis only}  
label 99;  
const norw = 11; {no. of reserved words}  
txmax = 100; {length of identifier table}  
nmax = 14; {max. no of digits in numbers}  
al = 10; {length of identifiers}  
type symbol = (nul, ident, number, plus, minus, times, slash, oddsym, eql, neg, lss, leq, gtr, geq, lparen, rparen, comma, semicolon, period, becomes, beginsym, endsym, ifsym, thensym, whilesym, dosym, callsym, consymsm, varsym, procsym);  
alfa = packed array [1.. al] of char;  
object = (constant, variable, procedure);  
var ch: char; {last character read}  
sym: symbol; {last symbol read}  
id: alfa; {last identifier read}  
num: integer; {last number read}  
cc: integer; {character count}  
ll: integer; {line length}  
kk: integer;  
line: array [1.. 81] of char;  
a: alfa;  
word: array [1.. norw] of alfa;  
wsym: array [1.. norw] of symbol;  
ssym: array [char] of symbol;  
table: array [0.. txmax] of record name: alfa; kind: object end ;

Program 5.4 (Continued)  
```prolog
procedure error (n: integer);  
begin writeln('':cc,' $\uparrow$  ', n:2); goto 99  
end {error};  
procedure getsym;  
var i,j,k:integer;  
procedure getch;  
begin if cc = ll then  
begin if eof(input) then  
begin write('PROGRAM INCOMPLETE'); goto 99  
end;  
ll := 0; cc := 0; write('');  
while -eoln(input) do  
begin ll := ll+1; read(ch); write(ch); line[ll] := ch  
end;  
writeln; ll := ll+1; read(line[ll])  
end;  
cc := cc+1; ch := line[cc]  
end {getch};  
begin {getsym}  
while ch = ' ' do getch;  
if ch in ['A'..'Z'] then  
begin {identifier or reserved word} k := 0;  
repeat if k < al then  
begin k := k+1; a[k] := ch  
end;  
getch  
until-(ch in ['A'..'Z', '0'..'9'])  
if k ≥ kk then kk := k else  
repeat a[kk] := ''; kk := kk-1  
until kk = k;  
id := a; i := 1; j := norw;  
repeat k := (i+j) div 2;  
if id ≤ word[k] then j := k-1;  
if id ≥ word[k] then i := k+1  
until i > j;  
if i-1 > j then sym := wsym[k] else sym := ident  
end else  
if ch in ['0'..'9'] then  
begin {number} k := 0; num := 0; sym := number;  
repeat num := 10*num + (ord(ch) - ord('0'));  
k := k+1; getch  
until-(ch in ['0'..'9']);  
if k > nmax then error (30)  
end else
```

Program 5.4 (Continued)  
```txt
if ch = ': then  
begin getch;  
if ch = ': then  
begin sym := becomes; getch  
end else sym := nul;  
end else  
begin sym := sSym[ch]; getch  
end  
end{getsym};  
procedure block (tx: integer);  
procedure enter (k: object);  
begin{enter object into table}  
tx := tx + 1;  
with table[tx] do  
begin name := id; kind := k;  
end  
end{enter};  
function position (id: alfa): integer;  
var i: integer;  
begin{find identifier id in table}  
table[0].name := id; i := tx;  
while table[i].name ≠ id do i := i-1;  
position := i  
end{position};  
procedure constdeclaration;  
begin if sym = ident then  
begin getsym;  
if sym = eql then  
begin getsym;  
if sym = number then  
begin enter (constant); getsym  
end  
else error (2)  
end else error (3)  
end else error (4)  
end{constdeclaration};  
procedure vardeclaration;  
begin if sym = ident then  
begin enter (variable); getsym  
end else error (4)  
end{vardeclaration};
```

```txt
procedure statement;  
var i: integer;  
procedure expression;  
procedure term;  
procedure factor;  
var i: integer;  
begin  
if sym = ident then  
begin i := position(id);  
if i = 0 then error (11) else  
if table[i].kind = procedure then error (21); getsym  
end else  
if sym = number then  
begin getsym  
end else  
if sym = lparen then  
begin getsym; expression;  
if sym = rparen then getsym else error (22)  
end  
else error (23)  
end {factor};  
begin {term} factor;  
while sym in [times, slash] do  
begin getsym; factor  
end  
end {term};  
begin {expression}  
if sym in [plus, minus] then  
begin getsym; term  
end else term;  
while sym in [plus, minus] do  
begin getsym; term  
end  
end {expression};  
procedure condition;  
begin  
if sym = oddsym then  
begin getsym; expression  
end else  
begin expression;  
if ¬(sym in [eql, neq, lss, leq, gtr, geq]) then error (20) else  
begin getsym; expression  
end  
end {condition};
```

Program 5.4 (Continued)  
```txt
begin{statement}   
if sym  $=$  ident then   
begin  $i\coloneqq$  position(id); if  $i = 0$  then error (11) else if table [i] .kind  $\neq$  variable then error (12); getsym; if sym  $=$  becomes then getsym else error (13); expression   
end else   
if sym  $=$  callsym then   
begin getsym; if sym  $\neq$  ident then error (14) else begin  $i\coloneqq$  position(id); if  $i = 0$  then error (11) else if table[i] .kind  $\neq$  procedure then error (15); getsym end   
end else   
if sym  $=$  ifsym then   
begin getsym; condition; if sym  $=$  thensym then getsym else error (16); statement;   
end else   
if sym  $=$  beginsym then   
begin getsym; statement; while sym  $=$  semicolon do begin getsym; statement end ; if sym  $=$  endsym then getsym else error (17)   
end else   
if sym  $=$  whilesym then   
begin getsym; condition; if sym  $=$  dosym then getsym else error (18); statement   
end   
end{statement};   
begin{block}   
if sym  $=$  constsym then   
begin getsym; constdeclaration; while sym  $=$  comma do begin getsym; constdeclaration end ; if sym  $=$  semicolon then getsym else error (5)   
end ;
```

```txt
if sym  $=$  varsym then   
begin getsym; vardeclaration; while sym  $=$  comma do begin getsym; vardeclaration end ; if sym  $=$  semicolon then getsym else error (5)   
end ;   
while sym  $=$  procsym do   
begin getsym; if sym  $=$  ident then begin enter (procedure); getsym end else error (4); if sym  $=$  semicolon then getsym else error (5); block  $(tx)$  . if sym  $=$  semicolon then getsym else error (5); end ; statement   
end  $\{block\}$
```

```txt
begin{main program}   
for ch  $\coloneqq$  A'to; do sSym[ch]  $\coloneqq$  nul;   
word[1]  $\coloneqq$  BEGIN ; word[2]  $\coloneqq$  CALL ;   
word[3]  $\coloneqq$  CONST; word[4]  $\coloneqq$  DO ;   
word[5]  $\coloneqq$  END ; word[6]  $\coloneqq$  IF ;   
word[7]  $\coloneqq$  ODD ; word[8]  $\coloneqq$  PROCEDURE;   
word[9]  $\coloneqq$  THEN ; word[10]  $\coloneqq$  VAR ;   
word[11]  $\coloneqq$  WHILE ;   
wsym[1]  $\coloneqq$  beginsym; wsym[2]  $\coloneqq$  callsym;   
wsym[3]  $\coloneqq$  constsym; wsym[4]  $\coloneqq$  dosym;   
wsym[5]  $\coloneqq$  endsym; wsym[6]  $\coloneqq$  ifsym;   
wsym[7]  $\coloneqq$  oddsym; wsym[8]  $\coloneqq$  procsym;   
wsym[9]  $\coloneqq$  thensym; wsym[10]  $\coloneqq$  varsym;   
wsym[11]  $\coloneqq$  whilesym;   
ssym['+']  $\coloneqq$  plus; ssym[-']  $\coloneqq$  minus;   
ssym['*']  $\coloneqq$  times; ssym['/']  $\coloneqq$  slash;   
ssym[''(]: lparen; ssym['')':  $=$  rparen;   
ssym['=']  $\coloneqq$  eql; ssym['',']  $\coloneqq$  comma;   
ssym['.']  $\coloneqq$  period; ssym[' $\neq$  ]  $\coloneqq$  neq;   
ssym['<']  $\coloneqq$  lss; ssym['>']  $\coloneqq$  gtr;   
ssym['≤']  $\coloneqq$  leq; ssym[' $\geq$  ]  $\coloneqq$  geq;   
ssym[';']  $\coloneqq$  semicolon;   
page(output);   
cc:  $= 0;$  ll:  $= 0;$  ch:  $= ^{\prime}$  ; kk:  $= a1;$  getsym; block (0);   
if sym  $\neq$  period then error (9);   
99: writeln   
end .
```

# 5.9. RECOVERING FROM SYNTACTIC ERRORS

Up to this point the parser had only the modest task of determining whether or not an input sequence of symbols belonged to a language. As a side product, the parser also discovered the inherent structure of a sentence. But as soon as an ill-formed construct was encountered, the parser's task was achieved, and the program could as well terminate. For practical compilers, this is of course no tenable proposition. Instead, a compiler must issue an appropriate error diagnostic and be able to continue the parsing process—probably to find further mistakes. A continuation is only possible either by making some likely assumption about the nature of the error and the intention of the author of the ill-formed program or by skipping over some subsequent part of the input sequence, or both. The art of choosing an assumption with a high likelihood of correctness is rather intricate. It has so far eluded any kind of successful formalization because formalizations of syntax and parsing do not take into account the many factors that strongly influence the human mind. For instance, it is a common error to omit inter-punctuation symbols such as the semicolon (not only in programming!), whereas it is highly improbable that one forgets to write a  $+$  operator in an arithmetic expression. The semicolon and plus symbol are merely terminal symbols without further distinction for the parser; for the human programmer, the semicolon has hardly a meaning and appears redundant at the end of a line, whereas the significance of an arithmetic operator is obvious beyond doubt. There are many more such considerations that have to go into the design of an adequate recovery system, and they all depend on the individual language and cannot be generalized in the framework of all context-free languages.

Nevertheless, there are some rules and hints that can be postulated and that have validity beyond the scope of a single language such as PL/0. Characteristically, perhaps, they are concerned equally much with the initial conception of a language as with the design of the recovery mechanism of its parser. First of all, it is abundantly clear that sensible recovery is much facilitated, or even made possible, only by a simple language structure. In particular, if upon diagnosing an error some part of the subsequent input is to be skipped (ignored), then it is mandatory that the language contains key words that are highly unlikely to be misused, and that may therefore serve to bring the parser back into step. PL/0 notably follows this rule: every structured statement begins with an unmistakable keyword such as begin, if, while, and the same holds for declarations; they are headed by var, const, or procedure. We shall therefore call this rule the keyword rule.

The second rule concerns the construction of the parser more directly. It is the characteristic of top-down parsing that goals are split up into

subgoals and that parsers call upon other parsers to tackle their subgoals. The second rule specifies that if a parser detects an error, it should not merely refuse to continue and report the happening back to its master parser. Instead, it should itself continue to scan text up to a point where some plausible analysis can be resumed. We shall therefore call this the don't panic rule. The programmatic consequence of this rule is that there will be no exit from a parser except through its regular termination point.

A possible strict interpretation of the don't panic rule consists of skipping input text upon detecting an illegal formation up to the next symbol that may correctly follow the currently parsed sentential construct. This implies that every parser know the set of its follow-symbols at the place of its present activation.

In the first refinement (or enrichment) step we shall therefore provide every parsing procedure with an explicit parameter  $fsys$  that specifies the possible follow-symbols. At the end of each procedure an explicit test is included to verify that the next symbol of the input text is indeed among those follow-symbols (if this condition is not already asserted by the logic of the program).

It would, however, be very shortsighted of us to skip the input text up to the next occurrence of such a follow-symbol under all circumstances. After all, the programmer may have mistakenly omitted exactly one symbol (say a semicolon); ignoring the entire text up to the next follow-symbol may be disastrous. We therefore augment these sets of symbols that terminate a possible skip by keywords that specifically mark the beginning of a construct not to be overlooked. The symbols passed as parameters to the parsing procedures are therefore stopping symbols rather than follow-symbols only. We may regard the sets of stopping symbols as being initialized by distinct key symbols and being gradually supplemented by legal follow-symbols upon penetration of the hierarchy of parsing subgoals. For flexibility, a general routine called test is introduced to perform the described verification. This procedure (5.17) has three parameters:

1. The set  $s1$  of admissible next symbols; if the current symbol is not among them, an error is at hand.  
2. A set  $s2$  of additional stopping symbols whose presence is definitely an error, but which should in no case be ignored and skipped.  
3. The number  $n$  of the pertinent error diagnostic.

$$
\begin{array}{l} \text {p r o c e d u r e t e s t (s 1 , s 2 : s y m s e t ; n : i n t e g e r) ;} \\ \text {b e g i n i f - (s y m i n s 1) t h e n} \\ \text {b e g i n e r r o r (n) ; s 1 := s 1 + s 2 ;} \\ \text {w h i l e - (s y m i n s 1) d o g e t s y m} \\ \text {e n d} \\ \text {e n d} \end{array} \tag {5.17}
$$

Procedure (5.17) may also be conveniently used at the entrance of parsing procedures to verify whether or not the current symbol is an admissible initial symbol. This is recommended in all cases in which a parsing procedure  $X$  is called unconditionally, such as in the statement

$$
\text {i f} \quad s y m = a _ {1} \text {t h e n} S _ {1} \text {e l s e}
$$

.

$$
\text {i f} \operatorname {s y m} = a _ {n} \text {t h e n} S _ {n} \text {e l s e} X
$$

which is the result of translation of the production

$$
A := a _ {1} S _ {1} | \dots | a _ {n} S _ {n} | X \tag {5.18}
$$

In these instances the parameter  $s1$  must be equal to the set of initial symbols of  $X$ , whereas  $s2$  is chosen as the set of the follow-symbols of  $A$  (see Table 5.2). The details of this procedure are given in Program 5.5, which represents the enriched version of Program 5.4. For the reader's convenience, the entire parser is listed again, with the exception of initializations of global variables and of the procedure getsym, all of which remain unchanged.

The scheme presented so far has the property of trying to recover, to fall back into step, by ignoring one or more symbols in the input text. This is an unfortunate strategy in all cases in which an error is caused by omission of a symbol. Experience shows that such errors are virtually restricted to symbols which have merely syntactic functions and do not represent an action. An example is the semicolon in PL/0. The fact that the follow-symbol sets are augmented by certain key words actually causes the parser to stop skipping symbols prematurely, thereby behaving as if a missing symbol had been inserted. This can be seen from the program part that parses compound statements shown in (5.19). It effectively “inserts” missing semicolons in front of key words. The set called statbegsys is the set of initial symbols of the construct “statement.”

if sym = beginsym then

begin getsym;

statement([semicolon, endsym] + fsys);

while sym in [semicolon] + statbegsys do

begin

gul if sum - semicolon then getsum else error: (5.19)

If sym — semicolon then getsym else statement([semicolon, endsym] + fsys)

end;

if sym = endsym then getsym else error

end

The degree of success with which this program diagnoses syntactic errors and recovers from unusual situations can be estimated by considering the PL/0 program (5.20). The listing represents an output delivered by Program

![](images/b96236b520603d7b9c7f585ab7ac5d696e9beb61959362257312b4f8a12b5435.jpg)  
Fig. 5.6 Modified compound statement syntax.

1. Use  $=$  instead of  $\coloneqq$  
2.  $=$  must be followed by a number.  
3. Identifier must be followed by  $= \cdot$  
4. const, var, procedure must be followed by an identifier.  
5. Semicolon or comma missing.  
6. Incorrect symbol after procedure declaration.  
7. Statement expected.  
8. Incorrect symbol after statement part in block.  
9. Period expected.  
10. Semicolon between statements is missing.  
11. Undeclared identifier.  
12. Assignment to constant or procedure is not allowed.  
13. Assignment operator  $\coloneqq$  expected.  
14. call must be followed by an identifier.  
15. Call of a constant or a variable is meaningless.  
16. then expected.  
17. Semicolon or end expected.  
18. do expected.  
19. Incorrect symbol following statement.  
20. Relational operator expected.  
21. Expression must not contain a procedure identifier.  
22. Right parenthesis missing.  
23. The preceding factor cannot be followed by this symbol.  
24. An expression cannot begin with this symbol.  
30. This number is too large.

Table 5.3 Error Messages of PL/0 Compiler.

5.5, and Table 5.3 lists a set of possible diagnostic messages corresponding to the error numbers in Program 5.5.

The following program (5.20) was obtained by the introduction of syntactic errors in (5.14) through (5.16).

```txt
const  $m = 7$  n=85   
var  $x,y,z,q,r;$  （20  $\uparrow 5$  （204   
procedure multiply;   
var  $a,b$
```

```txt
begin  $a\coloneqq u$  .  $b\coloneqq y$  .  $z\coloneqq 0$ $\uparrow 5$ $\uparrow 11$    
while  $b > 0$  do  
 $\uparrow 10$    
begin if odd  $\textit{\textbf{b}}$  do  $z\coloneqq z + a$  ..  $\uparrow 16$ $\uparrow 19$ $a\coloneqq 2a$  .  $b\coloneqq b / 2$  ..  $\uparrow 23$    
end   
end ;   
procedure divide   
var w;   
 $\uparrow 5$    
const two  $= 2$  ,three  $\coloneqq 3$  ..   
 $\uparrow 7$ $\uparrow 1$    
begin  $r = x$  .  $q\coloneqq 0$  .  $w\coloneqq y$  (5.20)  $\uparrow 13$ $\uparrow 24$    
while  $w\leq r$  do  $w\coloneqq two*w$    
while  $w > y$    
begin  $q\coloneqq (2*q;w\coloneqq w / 2)$  ..  $\uparrow 18$ $\uparrow 22$ $\uparrow 23$    
if  $w\leq r$  then begin  $r\coloneqq r - w q\coloneqq q + 1$ $\uparrow 23$    
end   
end   
end ;   
procedure gcd;   
var  $f,g$    
begin  $f\coloneqq x$  .  $g\coloneqq y$    
while  $f\neq g$  do  
 $\uparrow 17$    
begin if  $f <   g$  then  $g\coloneqq g - f$  .. if  $g <   f$  then  $f\coloneqq f - g$  ..   
 $z\coloneqq f$    
end ;
```

begin  
 $x := m; y := n$ ; call multiply;  
 $x := 25; y := 3$ ; call divide;  
 $x := 84; y := 36$ ; call gcd;  
call  $x$ ;  $x := \gcd$ ;  $gcd = x$ $\uparrow 15$ $\uparrow 21$ $\uparrow 12$ $\uparrow 13$ $\uparrow 24$   
end.  
 $\uparrow 17$ $\uparrow 5$ $\uparrow 7$   
PROGRAM INCOMPLETE

It should be clear that no scheme that reasonably efficiently translates correct sentences will also be able to handle all possible incorrect constructions in a sensible way. And why should it! Every scheme implemented with reasonable effort will fail, that is, will inadequately handle some misconstructions. The important characteristics of a good compiler, however, are that

1. No input sequence will cause the compiler to collapse.  
2. All constructs that are illegal according to the language definition are detected and marked.  
3. Errors that occur reasonably frequently and are true programmer's mistakes (caused by oversight or misunderstanding) are diagnosed correctly and do not cause any (or many) further stumbles of the compiler (so-called spurious error messages).

The presented scheme performs satisfactorily, although there is always room for improvement. Its merit is that it is built according to a few ground rules in a systematic fashion. The ground rules are merely supplemented by some choices of parameters based on heuristics and experience with actual use of the language.

Program 5.5 PL/0 Parser with Error Recovery.  
```txt
program PLO (input, output);  
{PL/0 compiler with syntax error recovery}
```

label 99;  
```txt
const norw  $= 11$  {no.ofreserved words}   
txmax  $= 100$  {length of identifier table}   
nmax  $= 14$  {max.no.of digits in numbers}   
al  $= 10$  {length of identifiers}   
type symbol  $\equiv$  (nul,ident,number,plus,minus,times,slash,oddsym, eql,neg,lss,leq,gtr,geq,lparen,rparen,comma,semicolon, period,becomes,beginsym,endsym,ifsym,thensym, whilesym,dosym,callsym,consyms,marsym,procsym); alfa  $=$  packed array [1..al] of char; object  $=$  (constant,variable,procedure); symset  $=$  set of symbol;   
var ch:char; {last character read}   
sym:symbol; {last symbol read}   
id:alfa; {last identifier read}   
num:integer; {last number read}   
cc:integer; {character count}   
ll:integer; {line length}   
kk:integer;   
line:array[1..81]of char;   
a:alfa;   
word:array[1..norw]ofalfa;   
wsym:array[1..norw]ofsymbol;   
ssym:array[char]ofsymbol;   
declbegsys,statabegsys,facbegsys:symset;   
table:array[0..txmax]of record name:alfa; kind:object   
end ;   
procedure error(n:integer);   
begin writeln(''cc,' $\uparrow$  ',n:2);   
end{error};   
procedure test(s1,s2:symset;n:integer);   
begin if-(sym in s1) then begin error(n);s1:=s1+s2; while-(sym in s1)dogetsym end   
end{test};
```

procedure block (tx:integer; fsys:symset);

procedure enter (k: object);

begin{enterobjectinto table}

$tx\coloneqq tx + 1;$

with table[tx] do

begin name:  $= id$  kind  $\coloneqq k$  end

end{enter}；

function position (id: alfa): integer;  
var i: integer;

begin{findidentifieridin table}

table[0].name := id;  $i \coloneqq tx$ ;

while table[i].name  $\neq$  id do  $i\coloneqq i - 1$  position  $\coloneqq i$

end{position}；

procedure constdeclaration;

begin if sym = ident then

begin getsym;

if sym in [eql, becomes] then

begin if  $sym =$  becomes then error (1);

getsym;

if sym = number then

begin enter (constant); getsym

end

else error (2)

end else error (3)

end else error (4)

end{constdeclaration}；

procedurevardeclaration;

begin if sym = ident then

begin enter (variable); getsym

end else error (4)

end{vardeclaration}；

procedure statement (fsys: symset);

var i:integer;

procedure expression (fsys: symset);

procedure term (fsys: symset);

procedure factor (fsys: symset);

var i:integer;

Program 5.5 (Continued)

```tcl
begin test (facbegsys, fsys, 24); while sym in facbegsys do begin if sym  $=$  ident then begin  $i\coloneqq$  position  $(id)$  . if  $i = 0$  then error (11) else if table[i] .kind  $=$  procedure then error (21); getsym end else if sym  $=$  number then begin getsym; end else if sym  $=$  lparen then begin getsym; expression ([rparen]  $^+$  fsys); if sym  $=$  rparen then getsym else error (22) end ; test(fsys, [lparen], 23) end end {factor}; begin {term} factor (fsys+ [times, slash]); while sym in [times, slash] do begin getsym; factor(fsys+ [times, slash]) end end {term};   
begin {expression} if sym in [plus ,minus] then begin getsym; term(fsys+[plus, minus]) end else term(fsys+[plus, minus]); while sym in [plus, minus] do begin getsym; term(fsys+[plus, minus]) end   
end {expression};   
procedure condition(fsys: symset);   
begin if sym  $=$  oddsym then begin getsym; expression(fsys); end else begin expression ([eql, neq, lss, gtr, leq, geq]  $^+$  fsys); if -sym in [eql, neq, lss, leq, gtr, geq]) then error (20) else begin getsym; expression (fsys) end   
end   
end{condition};
```

Program 5.5 (Continued)  
```txt
begin {statement} if sym  $=$  ident then begin  $i\coloneqq$  position(id); if  $i = 0$  then error (11) else if table[i].kind  $\neq$  variable then error (12); getsym; if sym  $=$  becomes then getsym else error (13); expression(fsys); end else if sym  $=$  callsym then begin getsym; if sym  $\neq$  ident then error (14) else begin  $i\coloneqq$  position(id); if  $i = 0$  then error (11) else if table[i].kind  $\neq$  procedure then error (15); getsym end end else if sym  $=$  ifsym then begin getsym; condition ([thensym, dosym]  $^+$  fsys); if sym  $=$  thensym then getsym else error (16); statement(fsys) end else if sym  $=$  beginsym then begin getsym; statement([semicolon, endsym]  $^+$  fsys); while sym in [semicolon]  $^+$  statbegsys do begin if sym  $=$  semicolon then getsym else error (10); statement([semicolon, endsym]  $^+$  fsys) end ; if sym  $=$  endsym then getsym else error (17) end else if sym  $=$  whilesym then begin getsym; condition([dosym]  $^+$  fsys); if sym  $=$  dosym then getsym else error (18); statement(fsys); end ; test(fsys, [ ], 19)   
end {statement};
```

```txt
begin{block} repeat if sym  $=$  constsym then begin getsym; repeat constdeclaration; while sym  $=$  comma do begin getsym; constdeclaration end ; if sym  $=$  semicolon then getsym else error (5) until sym  $\neq$  ident end ; if sym  $=$  varsym then begin getsym; repeat vardeclaration; while sym  $=$  comma do begin getsym; vardeclaration end ; if sym  $=$  semicolon then getsym else error (5) until sym  $\neq$  ident; end ; while sym  $=$  procsym do begin getsym; if sym  $=$  ident then begin enter (procedure); getsym end else error (4); if sym  $=$  semicolon then getsym else error (5); block  $(tx,[\mathrm{semicolon}] + fsys)$  . if sym  $=$  semicolon then begin getsym; test.statbegsys+ [ident, procsym], fsys, 6) end else error (5) end ; test.statbegsys+[ident], declbegsys, 7) until-(sym in declbegsys); statement([semicolon, endsym] + fsys); test(fsys, [], 8);   
end {block};   
begin{main program} ... Initialization (see Program 5.4)... cc := 0; ll := 0; ch := ' '; kk := al; getsym; block (0, [period] + declbegsys + statbegsys); if sym  $\neq$  period then error (9);   
99: writeln end .
```

# 5.10. A PL/0 PROCESSOR

It is indeed remarkable that the PL/0 compiler was so far developed without any knowledge of the machine for which it was supposed to generate code. But why should the structure of an object machine influence the parsing and error recovery scheme of a compiler! In fact, it must not do so. Instead, the proper scheme for code generation for any computer should be superimposed on the existing parser by the method of stepwise refinement of the existing program. Since we are about to do this, it becomes necessary to select a processor for which to compile.

In order to keep the description of the compiler reasonably simple and free from extraneous considerations of peculiar properties of a real, existing processor, we shall postulate a computer of our own choice, specifically tailored to the needs of PL/0. Since this processor does not really exist (in hardware), it is a hypothetical processor; it will be called the  $PL / 0$  machine.

It is not the aim of this section to explain the detailed reasoning that led to the choice of exactly this kind of machine architecture. Instead, it is to serve as a descriptive manual consisting of an intuitive introduction, followed by a detailed definition of the processor in the form of an algorithm. This formalization may serve as an example for accurate and detailed algorithmic descriptions of actual processors. The algorithm interprets PL/0 instructions sequentially, and is called an interpreter.

The PL/0 machine consists of two stores: an instruction register and three address registers. The program store, called code, is loaded by the compiler and remains unchanged during interpretation of the code. It can then be considered as a read-only store. The data store  $S$  is organized as a stack, and all arithmetic operators operate on the two elements on top of the stack, replacing their operands by a result. The top element is addressed (indexed) by the top stack register  $T$ . The instruction register  $I$  contains the instruction that is currently being interpreted. The program address register  $P$  designates the next instruction to be fetched for interpretation.

Every procedure in PL/0 may contain local variables. Since procedures may be activated recursively, storage for these local variables may not be allocated before the actual procedure call. Hence, the data segments for individual procedures are stacked up consecutively in the stack store S. Since procedure activations strictly obey the first-in-last-out scheme, the stack is the appropriate storage allocation strategy. Every procedure owns some internal information of its own, namely, the program address of its call (the so-called return address), and the address of the data segment of its caller. These two addresses are needed for proper resumption of program execution after termination of the procedure. They can be understood as internal or implicit local variables allocated in the procedure's data segment.

We call them the return address  $RA$  and the dynamic link  $DL$ . The origin of the dynamic link, that is, the address of the most recently allocated data segment, is retained in the base address register  $B$ .

Since the actual allocation of storage takes place during execution (interpretation) time, the compiler cannot equip the generated code with absolute addresses. Since it can only determine the location of variables within a data segment, it is capable of providing relative addresses only. The interpreter has to add to this so-called displacement to the base address of the appropriate data segment. If a variable is local to the procedure currently being interpreted, then this base address is given by the  $B$  register. Otherwise, it must be obtained by descending the chain of data segments. The compiler, however, can only know the static depth of an access path, whereas the dynamic link chain maintains the dynamic history of procedure activations. Unfortunately, these two access paths are not necessarily the same.

For example, assume that a procedure  $A$  calls a procedure  $B$  declared local to  $A$ ,  $B$  calls  $C$  declared local to  $B$ , and  $C$  calls  $B$  (recursively). We say that  $A$  is declared at level 1,  $B$  at level 2,  $C$  at level 3 (see Fig. 5.7). If a variable  $a$  declared in  $A$  is to be accessed in  $B$ , then the compiler knows that there exists a level difference of 1 between  $B$  and  $A$ . Descending one step along the dynamic link chain, however, would result in an access to a variable local to  $C!$

![](images/6f4db7d1e3569bcf8b74ea65e1f75f2d20ac1dda743e5f1979e8e91e66e41795.jpg)  
Fig. 5.7 Stack of PL/0 machine.

Hence, it is plain that a second link chain has to be provided that properly links data segments in the way the compiler can see the situation. We call this the static link  $SL$ .

Addresses are therefore generated as pairs of numbers indicating the static level difference and the relative displacement within a data segment. We assume that each location of the data store is capable of holding an address or an integer.

The instruction set of the PL/0 machine is tuned to the requirements of the PL/0 language. It includes the following orders:

1. An instruction to load numbers (liters) onto the stack (LIT).  
2. An instruction to fetch variables onto the top of the stack (LOD).  
3. A store instruction corresponding to assignment statements (STO).  
4. An introduction to activate a subroutine corresponding to a procedure call (CAL).  
5. An instruction to allocate storage on the stack by incrementing the stack pointer  $T$  (INT).  
6. Instructions for unconditional and conditional transfer of control, used in if- and while statements (JMP, JPC).  
7. A set of arithmetic and relational operators (OPR).

The format of instructions is determined by the need for three components, namely, an operation code  $f$  and a parameter consisting of one or two parts (see Fig. 5.8). In the case of operators the parameter  $a$  determines the identity of the operator; in the other cases it is either a number (LIT, INT), a program address (JMP, JPC, CAL), or a data address (LOD, STO).

<table><tr><td>f</td><td>∅</td><td>a</td></tr></table>

Fig. 5.8 Instruction format.

The details of operation of the PL/0 machine should be evident from the procedure called interpret that is part of Program 5.6, which combines the completed compiler with the interpreter into a system that translates and subsequently executes PL/0 programs. The modification of this program to generate code for an existing computer is left as an exercise for the interested reader. The resulting expansion of the compiler program may be taken as a measure of the appropriateness of the chosen computer for the present task.

There is no doubt that the presented PL/0 computer could be expanded into a more sophisticated organization in order to make certain operations more efficient. One instance is the chosen addressing mechanism. The presented solution was chosen because of its inherent simplicity and because all improvements must essentially be based on it and derived from it.

# 5.11. CODE GENERATION

In order to be able to assemble an instruction, the compiler must know its operation code and its parameter, which is a literal number or an address. These values are associated by the compiler itself with the respective identifiers. This association is performed upon processing the declaration of constants, variables, and procedures. For this purpose, the table containing the identifiers is expanded to contain the attributes associated with each identifier. If an identifier denotes a constant, its attribute is the constant value; if the identifier denotes a variable, the attribute is its address, consisting of a displacement and a level; and if the identifier denotes a procedure, then its attributes are the procedure's entry address and its level. The corresponding extension of the declaration of the variable table is shown in Program 5.6. It is a noteworthy example of a stepwise refinement (or enrichment) of a data declaration progressing simultaneously with the refinement of the statement part.

Whereas the constant values are provided by the program text, it is the compiler's task to determine addresses on its own. PL/0 is sufficiently simple to make sequential allocation of variables and code the obvious choice. Hence, every variable declaration is processed by incrementing a data allocation index by 1 (since each variable occupies by definition of the PL/0 machine exactly one storage cell). The data allocation index  $dx$  is to be initialized upon starting the compilation of a procedure, reflecting the fact that its data segment starts empty. [Actually,  $dx$  is given the initial value 3 since each data segment contains at least the three internal variables  $RA$ ,  $DL$ ,  $SL$  (see preceding section).] The appropriate computations to determine the identifiers' attributes are included in the procedure enter which is used to enter new identifiers into the table.

With this information about operands at hand, generating the actual code is a rather simple affair. Because of the convenient stack organization of the PL/0 machine, there exists practically a one-to-one correspondence between operands and operators in the source language and instructions in the target code. The compiler has merely to perform a suitable resequencing into postfix form. By "postfix form" is meant that operators always follow their operands instead of being embedded between the operands as in the conventional infix form. The postfix form is sometimes also called Polish form (after its originator Lukasciewicz) or parenthesis-free form since it makes parentheses superfluous. Some correspondences between infix and postfix forms of expressions are shown in Table 5.4 (see also Sect. 4.4.2).

The very simple technique of performing this transformation is shown by the procedures expression and term in Program 5.6. It is merely a matter

<table><tr><td>Infix Form</td><td>Postfix Form</td></tr><tr><td>x + y(x - y) + zx - (y + z)x*(y + z)*w</td><td>xy+xy - z+xyz+-xyz + *w*</td></tr></table>

of delaying the transmission of the arithmetic operator. At this point the reader should verify that the presented arrangement of parsing procedures also takes care of an appropriate interpretation of the conventional priority rules among the various operators.

A slightly less trivial matter is the translation of conditional and repetitive statements. In this case the generation of jump instructions is necessary, for which at times the destination address is still unknown. If one insists on a strictly sequential production of instructions in the form of an output file, then a two-pass compiler scheme is necessary. The second pass then assumes the task of supplementing the incomplete jump instructions with their destination addresses. An alternative solution adopted by the present compiler is to place the instructions into an array and essentially retaining them in directly accessible store. This method allows supplementing the missing addresses as soon as they become known. This operation is commonly called a fixup.

The only additional operation that has to be performed when issuing such a forward jump is to retain its location, i.e., its index in the program store. This address is then used to locate the incomplete instruction at the time of the fixup. The details are again evident from Program 5.6 (see routines processing if- and while statements). The patterns of code generated for the if- and while statements are as follows ( $L1$  and  $L2$  stand for code addresses):

Table 5.4 Expressions in Infix and Postfix Form.  

<table><tr><td colspan="2">if C then S</td><td colspan="2">while C do S</td></tr><tr><td colspan="2">code for condition C</td><td>L1:</td><td>code for C</td></tr><tr><td colspan="2">JPC L1</td><td></td><td>JPC L2</td></tr><tr><td colspan="2">code for statement S</td><td></td><td>code for S</td></tr><tr><td>L1:</td><td>...</td><td></td><td>JMP L1</td></tr><tr><td></td><td></td><td>L2:</td><td>...</td></tr></table>

For convenience, an auxiliary procedure called gen is introduced. Its purpose is to assemble and emit an instruction according to its three parameters. It automatically increments the code index  $cx$  which designates the location of the next instruction to be issued.

As an example, the code emitted by compiling the multiplication routine (5.14) is listed below in mnemonic form. The comments on the right-hand side are merely added for explanatory purposes.

<table><tr><td>2</td><td>INT</td><td>0,5</td><td>allocate space for links and local variables</td></tr><tr><td>3</td><td>LOD</td><td>1,3</td><td>x</td></tr><tr><td>4</td><td>STO</td><td>0,3</td><td>a</td></tr><tr><td>5</td><td>LOD</td><td>1,4</td><td>y</td></tr><tr><td>6</td><td>STO</td><td>0,4</td><td>b</td></tr><tr><td>7</td><td>LIT</td><td>0,0</td><td>0</td></tr><tr><td>8</td><td>STO</td><td>1,5</td><td>z</td></tr><tr><td>9</td><td>LOD</td><td>0,4</td><td>b</td></tr><tr><td>10</td><td>LIT</td><td>0,0</td><td>0</td></tr><tr><td>11</td><td>OPR</td><td>0,12</td><td>&gt;</td></tr><tr><td>12</td><td>JPC</td><td>0,29</td><td></td></tr><tr><td>13</td><td>LOD</td><td>0,4</td><td>b</td></tr><tr><td>14</td><td>OPR</td><td>0,7</td><td>odd</td></tr><tr><td>15</td><td>JPC</td><td>0,20</td><td></td></tr><tr><td>16</td><td>LOD</td><td>1,5</td><td>z</td></tr><tr><td>17</td><td>LOD</td><td>0,3</td><td>a</td></tr><tr><td>18</td><td>OPR</td><td>0,2</td><td>+</td></tr><tr><td>19</td><td>STO</td><td>1,5</td><td>z</td></tr><tr><td>20</td><td>LIT</td><td>0,2</td><td>2</td></tr><tr><td>21</td><td>LOD</td><td>0,3</td><td>a</td></tr><tr><td>22</td><td>OPR</td><td>0,4</td><td>*</td></tr><tr><td>23</td><td>STO</td><td>0,3</td><td>a</td></tr><tr><td>24</td><td>LOD</td><td>0,4</td><td>b</td></tr><tr><td>25</td><td>LIT</td><td>0,2</td><td>2</td></tr><tr><td>26</td><td>OPR</td><td>0,5</td><td>/</td></tr><tr><td>27</td><td>STO</td><td>0,4</td><td>b</td></tr><tr><td>28</td><td>JMP</td><td>0,9</td><td></td></tr><tr><td>29</td><td>OPR</td><td>0,0</td><td>return</td></tr></table>

Code corresponding to PL/0 procedure 5.14.

Many tasks in compiling programming languages are considerably more complex than the ones presented in the PL/0 compiler for the PL/0 machine [5-4]. Most of them are much more resistant to being neatly organized. The reader trying to extend the presented compiler in either direction toward a more powerful language or a more conventional computer will soon realize the truth of this statement. Nevertheless, the basic approach toward designing a complex program presented here retains its validity, and even increases its value when the task grows more complicated and more sophisticated. It has, in fact, been successfully used in the construction of large compilers [5-1 and 5-9].

```txt
program pl0(input, output);  
{pl/0 compiler with code generation}  
label 99;  
const norw = 11; {no. of reserved words}  
txmax = 100; {length of identifier table}  
nmax = 14; {max. no. of digits in numbers}  
al = 10; {length of identifiers}  
amax = 2047; {maximum address}  
levmax = 3; {maximum depth of block nesting}  
cxmax = 200; {size of code array}
```

```txt
type symbol =  
(nul,ident,number,plus,minus,times,slash,oddsym,  
eq1,neq,lss,leq,gtr,geq,lparen,rparen,comma,semicolon,  
period,becomes,beginsym,endsym,ifsym,thensym,  
whilesym,dosym,callsym,constsym,varsym,procsym);  
alfa = packed array [1..al] of char;  
object = (constant,varible,proc);  
symset = set of symbol;  
fct = (lit,opr,lod,stocal,int,jmp,jpc); {functions}  
instruction = packed record  
f: fct; {function code}  
l: 0..levmax; {level}  
a: 0..amax {displacement address}  
end;
```

```csv
{ lit 0,a : load constant a  
opr 0,a : execute operation a  
lod 1,a : load variable 1,a  
sto 1,a : store variable 1,a  
cal 1,a : call procedure a at level 1  
int 0,a : increment t-register by a  
jmp 0,a : jump to a  
jpc 0,a : jump conditional to a }
```

```txt
var ch: char; {last character read} sym: symbol; {last symbol read} id: alfa; {last identifier read} num: integer; {last number read} cc: integer; {character count} ll: integer; {line length} kk, err: integer; cx: integer; {code allocation index} line: array [1..81] of char; a: alfa; code: array [0..cxmax] of instruction; word: array [1..norw] of alfa; wsym: array [1..norw] of symbol; sSym: array [char] of symbol; mnemonic: array [fct] of packed array [1..5] of char; declbegsys, statbegsys, facbegsys: symset table: array [0..txmax] of record name: alfa; case kind: object of constant: (val: integer); variable, proc: (level, adr: int end;
```

```txt
procedure error(n: integer);  
begin writeln('\*\*\*\*', '': cc-1, '\^', n: 2); err := err+1  
end {error};
```

```txt
procedure getsym;  
var i,j,k:integer;
```

```txt
procedure getch;   
begin if cc = 11 then begin if eof(input) then begin write(' program incomplete'); goto 99 end; 11 := 0; cc := 0; write(cx: 5, ''); while not eoln(input) do begin 11 := 11+1; read(ch); write(ch); line[11] := ch end; writeln; readln; 11 := 11 + 1; line[11] := ''; end; cc := cc+1; ch := line[cc] end {getch};   
begin {getsym} while ch = ' ' do getch; if ch in ['a'..'z'] then begin {identifier or reserved word} k := 0; repeat if k < al then begin k := k+1; a[k] := ch end; getch; until not(ch in ['a'..'z', '0'..'9']) if k >= kk then kk := k else repeat a[kk] := ' '; kk := kk-1 until kk = k; id := a; i := 1; j := norw; repeat k := (i+j) div 2; if id <= word[k] then j := k-1; if id >= word[k] then i := k+1 until i > j; if i-1 > j then sym := wsym[k] else sym := ident end else if ch in ['0'..'9'] then begin {number} k := 0; num := 0; sym := number; repeat num := 10*num + (ord(ch)-ord('0')); k := k+1; getch until not(ch in ['0'..'9']) if k > nmax then error(30) end else if ch = ': ' then begin getch; if ch = ': ' then begin sym := becomes; getch end else sym := nul; end else begin sym := ssym[ch]; getch end {getsym};   
procedure gen(x: fmt; y, z: integer);   
begin if cx > cmax then begin write(' program too long'); goto 99 end; with code[cx] do begin f := x; l := y; a := z end; cx := cx + 1   
end {gen};
```

```vhdl
procedure test(s1,s2: symset; n: integer);  
begin if not.sym in s1) then  
    begin error(n); s1 := s1 + s2;  
    while not.sym in s1) do getsym  
    end  
end {test};  
procedure block(lev,tx: integer; fsys: symset);  
var dx: integer; {data allocation index}  
tx0: integer; {initial table index}  
cx0: integer; {initial code index}  
procedure enter(k: object);  
begin {enter object into table}  
tx := tx + 1;  
with table[tx] do  
begin name := id; kind := k;  
case k of  
    constant: begin if num >amax then  
        begin error(30); num := 0 end;  
        val := num  
    end;  
    variable: begin level := lev; adr := dx; dx := dx + 1;  
    end;  
proc: level := lev  
    end  
end {enter};  
function position(id: alfa): integer;  
var i: integer;  
begin {find identifier id in table}  
table[0].name := id; i := tx;  
while table[i].name <> id do i := i-1;  
position := i  
end {position};  
procedure constdeclaration;  
begin if sym = ident then  
    begin getsym;  
    if sym in [eq1, becomes] then  
        begin if sym = becomes then error(1);  
        getsym;  
        if sym = number then  
            begin enter(constant); getsym  
            end  
        else error(2)  
        end else error(3)  
        end else error(4)  
    end {constdeclaration};  
procedure vardeclaration;  
begin if sym = ident then  
    begin enter(variable); getsym  
    end else error(4)  
end {vardeclaration};  
procedure listcode;  
var i: integer;  
begin {list code generated for this block}  
for i := cx0 to cx-1 do  
    with code[i] do
```

```julia
writeln(i:5, mnemonic[f]:5, 1:3, a:5)
end{(listcode)};
procedure statement(fsys: symset);
var i, cx1, cx2: integer;
procedure expression(fsys: symset);
var addop: symbol;
procedure term(fsys: symset);
var mulop: symbol;
procedure factor(fsys: symset);
var i: integer;
begin test(facbegsys, fsys, 24);
while sym in facbegsys do
begin
    if sym = ident then
        begin i := position(id);
        if i = 0 then error(11) else
            with table[i] do
                case kind of
                    constant: gen(lit, 0, val);
                    variable: gen(lod, lev-level, adr);
                    proc: error(21)
                    end;
                getsym
                    end else
                    if sym = number then
                        begin if num >amax then
                            begin error(30); num := 0
                            end;
                        gen(lit, 0, num); getsym
                    end else
                    if sym = lparen then
                        begin getsym; expression([rparen]+fsys);
                        if sym = rparen then getsym else error(22)
                        end;
                        test(fsys, [lparen], 23)
                        end
end{factor};
begin{term} factor(fsys+[times, slash]);
while sym in [times, slash] do
begin mulop:=sym; getsym; factor(fsys+[times, slash]);
if mulop=times then gen(opr, 0, 4) else gen(opr, 0, 5)
end
end{term};
begin{expression}
if sym in [plus, minus] then
begin addop := sym; getsym; term(fsys+[plus, minus]);
if addop = minus then gen(opr, 0, 1)
end else term(fsys+[plus, minus]);
while sym in [plus, minus] do
begin addop := sym; getsym; term(fsys+[plus, minus]);
if addop=plus then gen(opr, 0, 2) else gen(opr, 0, 3)
end
end{expression};
procedure condition(fsys: symset);
var relop: symbol;
begin
if sym = oddsym then
begin getsym; expression(fsys); gen(opr, 0, 6)
```

```asm
end else
begin expression([eq1, neq, lss, gtr, leq, geq] + fsys;
if not(sym in [eq1, neq, lss, leq, gtr, geq]) then
    error(20) else
begin relop := sym; getsym; expression(fsys);
    case relop of
        eq1: gen(opr, 0, 8);
        neq: gen(opr, 0, 9);
        lss: gen(opr, 0, 10);
        geq: gen(opr, 0, 11);
        gtr: gen(opr, 0, 12);
        leq: gen(opr, 0, 13);
    end
end
end
end
end {condition};
begin {statement}
if sym = ident then
begin i := position(id);
if i = 0 then error(l1) else
if table[i].kind <> variable then
begin {assignment to non-variable} error(l2); i := end;
getsym; if sym = becomes then getsym else error(l3);
expression(fsys);
if i <> 0 then
with table[i] do gen(sto, lev-level, adr)
end else
if sym = callsym then
begin getsym;
if sym <> ident then error(l4) else
begin i := position(id);
if i = 0 then error(l1) else
with table[i] do
if kind=proc then gen(cal, lev-level, adr)
else error(l5);
getsym
end
end else
if sym = ifsym then
begin getsym; condition([thensym, dosym] + fsys);
if sym = thensym then getsym else error(l6);
cx1 := cx; gen(jpc, 0, 0);
statement(fsys); code[cx1].a := cx
end else
if sym = beginsym then
begin getsym; statement([semicolon, endsym] + fsys);
while sym in [semicolon] + statbegsys do
begin
if sym = semicolon then getsym else error(l7);
statement([semicolon, endsym] + fsys)
end;
if sym = endsym then getsym else error(l8);
end else
if sym = whilesym then
begin cx1 := cx; getsym; condition([dosym] + fsys);
cx2 := cx; gen(jpc, 0, 0);
if sym = dosym then getsym else error(l9);
statement(fsys); gen(jmp, 0, cx1); code[cx2].a := cx
```

```txt
test(fsys, [], 19)  
end {statement};  
begin {block} dx:=3; tx0:=tx; table[tx].adr:=cx; gen(jmp,0,0); if lev > levmax then error(32); repeat if sym = constsym then begin getsym; repeat constdeclaration; while sym = comma do begin getsym; constdeclaration end; if sym = semicolon then getsym else error(5) until sym <> ident end; if sym = varsym then begin getsym; repeat vardeclaration; while sym = comma do begin getsym; vardeclaration end; if sym = semicolon then getsym else error(5) until sym <> ident; end; while sym = procsym do begin getsym; if sym = ident then begin enter(proc); getsym end else error(4); if sym = semicolon then getsym else error(5); block(lev+1, tx, [semicolon]+fsys); if sym = semicolon then begin getsym; test(statbegsys+[ident,procsym],fsys,6) end else error(5) end; test(statbegsys+[ident], declbegsys,7) until not.sym in declbegsys); code[table[tx0].adr].a := cx; with table[tx0] do begin adr := cx; {start adr of code} end; cx0 := 0{cx}; gen(int,0,dx); statement([[semicolon, endsym]+fsys]); gen(opr,0,0); {return} test(fsys, [], 8); listcode; end {block};
```

```txt
procedure interpret;  
const stacksize = 500;  
var p, b, t: integer; {program-, base-, topstack-registers}  
i: instruction; {instruction register}  
s: array [1..stacksize] of integer; {datastore}  
function base(l: integer): integer;  
var bl: integer;  
begin bl := b; {find base l levels down}  
while l > 0 do  
begin bl := s[b1]; l := l - 1  
end;
```

```vhdl
base := b1
end {base};
begin writeln(' start pl/0');
t := 0; b := 1; p := 0;
s[1] := 0; s[2] := 0; s[3] := 0;
repeat i := code[p]; p := p + 1;
with i do
case f of
lit: begin t := t + 1; s[t] := a
end;
opr: case a of {operator}
0: begin {return}
t := b - 1; p := s[t + 3]; b := s[t + 2];
end;
1: s[t] := -s[t];
2: begin t := t - 1; s[t] := s[t] + s[t + 1]
end;
3: begin t := t - 1; s[t] := s[t] - s[t + 1]
end;
4: begin t := t - 1; s[t] := s[t] * s[t + 1]
end;
5: begin t := t - 1; s[t] := s[t] div s[t + 1]
end;
6: s[t] := ord(odds(t));
8: begin t := t - 1; s[t] := ord(s[t] = s[t + 1])
end;
9: begin t := t - 1; s[t] := ord(s[t] <> s[t + 1])
end;
10: begin t := t - 1; s[t] := ord(s[t] < s[t + 1])
end;
11: begin t := t - 1; s[t] := ord(s[t] >= s[t + 1])
end;
12: begin t := t - 1; s[t] := ord(s[t] > s[t + 1])
end;
13: begin t := t - 1; s[t] := ord(s[t] <= s[t + 1])
end;
lod: begin t := t + 1; s[t] := s[base(l) + a]
end;
sto: begin s[base(l) + a] := s[t]; writeln(s[t]); t := t - 1
end;
cal: begin {generate new block mark}
s[t + 1] := base(l); s[t + 2] := b; s[t + 3] := p;
b := t + 1; p := a
end;
int: t := t + a;
jmp: p := a;
jpc: begin if s[t] = 0 then p := a; t := t - 1
end
end {with, case}
until p = 0;
write(' end pl/0');
end {interpret};
```

```txt
begin{main program} for ch := chr(0) to chr(255) do sSym[ch] := nul; word[1] := 'begin' ; word[2] := 'call' ; word[3] := 'const' ; word[4] := 'do' ; word[5] := 'end' ; word[6] := 'if' ; word[7] := 'odd' ; word[8] := 'procedure' ;
```

```txt
word[9] := 'then'; word[10] := 'var';  
word[11] := 'while';  
wsym[1] := beginsym; wsym[2] := callsym;  
wsym[3] := constsym; wsym[4] := dosym;  
wsym[5] := endsym; wsym[6] := ifsym;  
wsym[7] := oddsym; wsym[8] := procsym;  
wsym[9] := thensym; wsym[10] := varsym;  
wsym[11] := whilesym;  
ssym['+'] := plus; ssym['-'] := minus;  
ssym['*'] := times; ssym['/'] := slash;  
ssym['(')] := lparen; ssym['')'] := rparen;  
ssym['='] := eql; ssym[',''] := comma;  
ssym['.'] := period; ssym[#'] := neq;  
ssym['<'] := lss; ssym['>'] := gtr;  
ssym['[]'] := leq; ssym[''] := geq;  
ssym[' ';''] := semicolon;  
mnemonic[lit] := 'lit'; mnemonic[opr] := 'opr';  
mnemonic[lod] := 'lod'; mnemonic[sto] := 'sto';  
mnemonic[cal] := 'cal'; mnemonic[int] := 'int';  
mnemonic[jmp] := 'jmp'; mnemonic[jpc] := 'jpc';  
declbegsys := [constsym, varsym, procsym];  
statbegsys := [beginsym, callsym, ifsym, whilesym];  
facbegsys := [ident, number, lparen];  
page(output); err := 0;  
cc := 0; cx := 0; ll := 0; ch := ''; kk := al; getsym;  
block(0, 0, [period] + declbegsys + statbegsys);  
if sym <> period then error(9);  
if err = 0 then interpret else write('errors in pl/0 program');  
99: writeln end.
```
