
# Table of Contents

1.  [README](#orge660671)
    1.  [Getting started](#orgadbee09)
    2.  [Milestones](#orgbbf1f1d)
    3.  [Current progress](#org021712f)
    4.  [Must have](#org01d4559)
        1.  [Scalability](#orgae1f3bb)
        2.  [Instanceable zones](#orgd086ede)
        3.  [Phasing](#org1800943)
        4.  [Region of interest streaming ONLY](#org7b544fd)
    5.  [BombShell Protocol (BBP)](#org8d2b40e)
        1.  [Naming conventions](#org616c9a4)
        2.  [Sending a BBP packet](#orgcdbed6f)
        3.  [Protocol Package Types (OpCodes)](#org18d1a6a)
        4.  [Encoding](#orgefbac03)
        5.  [Protocol packets](#org0132244)
    6.  [Test Client](#orgcb86e87)


<a id="orge660671"></a>

# README


<a id="orgadbee09"></a>

## Getting started

The server listens on localhost on port **47331**.
You need to have the latest stable Rust installed along with cargo.

-   Inside the repository, build the server and the test client:
    
        cargo build
-   Then run server:
    
        target/debug/server
-   Run test client:
    
        target/debug/test_client


<a id="orgbbf1f1d"></a>

## Milestones

1.  Login with admin/admin
2.  Stream player joined (spawn player with name)
3.  Other PCC movement/action streaming
4.  Map interpretation (collision/blocking)
5.  NPC interaction/patrol basic implementation


<a id="org021712f"></a>

## Current progress

-   [ ] M1


<a id="org01d4559"></a>

## Must have


<a id="orgae1f3bb"></a>

### Scalability


<a id="orgd086ede"></a>

### Instanceable zones


<a id="org1800943"></a>

### Phasing


<a id="org7b544fd"></a>

### Region of interest streaming ONLY


<a id="org8d2b40e"></a>

## BombShell Protocol (BBP)


<a id="org616c9a4"></a>

### Naming conventions

-   Client to Server: Recv
-   Server to Client: Send


<a id="orgcdbed6f"></a>

### Sending a BBP packet

<table border="2" cellspacing="0" cellpadding="6" rules="groups" frame="hsides">


<colgroup>
<col  class="org-left" />

<col  class="org-left" />
</colgroup>
<thead>
<tr>
<th scope="col" class="org-left">0-2 bytes</th>
<th scope="col" class="org-left">OPCODE</th>
</tr>
</thead>

<tbody>
<tr>
<td class="org-left">2-1400 bytes</td>
<td class="org-left">Protocol specific data</td>
</tr>
</tbody>
</table>


<a id="org18d1a6a"></a>

### Protocol Package Types (OpCodes)

Protocol package opcodes are 2 bytes numbers (u16 in Rust). They are incremented from 1 as the following:

1.  AUTH


<a id="orgefbac03"></a>

### Encoding

1.  Numbers

    All numbers are encoded as little-endian bytes. 

2.  Strings

    Strings are encoded as UTF-8 bytes as following:
    
    <table border="2" cellspacing="0" cellpadding="6" rules="groups" frame="hsides">
    
    
    <colgroup>
    <col  class="org-left" />
    
    <col  class="org-left" />
    </colgroup>
    <thead>
    <tr>
    <th scope="col" class="org-left">0-4 bytes</th>
    <th scope="col" class="org-left">Length of bytes (not length of the string!)</th>
    </tr>
    </thead>
    
    <tbody>
    <tr>
    <td class="org-left">4-? bytes</td>
    <td class="org-left">UTF-8 encoded bytes</td>
    </tr>
    </tbody>
    </table>


<a id="org0132244"></a>

### Protocol packets

1.  AUTH

    <table border="2" cellspacing="0" cellpadding="6" rules="groups" frame="hsides">
    
    
    <colgroup>
    <col  class="org-left" />
    
    <col  class="org-left" />
    </colgroup>
    <tbody>
    <tr>
    <td class="org-left">0-2 bytes</td>
    <td class="org-left">OPCODE</td>
    </tr>
    
    
    <tr>
    <td class="org-left">2-6 bytes</td>
    <td class="org-left">Username str length</td>
    </tr>
    
    
    <tr>
    <td class="org-left">6-X bytes</td>
    <td class="org-left">Username str bytes</td>
    </tr>
    
    
    <tr>
    <td class="org-left">X-X+4 bytes</td>
    <td class="org-left">Hash str byte length</td>
    </tr>
    
    
    <tr>
    <td class="org-left">X+4-Y bytes</td>
    <td class="org-left">Hash str bytes</td>
    </tr>
    </tbody>
    </table>


<a id="orgcb86e87"></a>

## Test Client

The test client can send BBP packages based on prompt input.
Use prefixes to encode the corresponding messages:

-   ptc: protocol packet type (opcode)
-   str: str encoded bytes
-   u16: 2 bytes number
-   u32: 4 bytes number

The authentication packet could be sent as the following example shows:

> > ptc1
> > stradmin
> > strtesthash12345
> > send

The **send** command flushes the bytes and sends them to the backend.

