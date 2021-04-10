import 'package:flutter/material.dart';
import 'package:google_fonts/google_fonts.dart';

import 'logo.dart';

void main() {
  runApp(MyApp());
}

class MyApp extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      theme: ThemeData(
        primarySwatch: MaterialColor(
          0xffef0164,
          <int, Color>{
            50: Color(0xFFE8F5E9),
            100: Color(0xFFC8E6C9),
            200: Color(0xFFA5D6A7),
            300: Color(0xFF81C784),
            400: Color(0xFF66BB6A),
            500: Color(0xffef0164), // Primary value.
            600: Color(0xFF43A047),
            700: Color(0xFF388E3C),
            800: Color(0xFF2E7D32),
            900: Color(0xFF1B5E20),
          },
        ),
        accentColor: Colors.green,
      ),
      home: IdleScreen(),
    );
  }
}

class IdleScreen extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: Colors.white,
      body: Center(
        child: Column(
          children: [
            Spacer(),
            Logo(),
            SizedBox(height: 16),
            // ElevatedButton(
            //   onPressed: () {},
            //   style: ButtonStyle(
            //     elevation: MaterialStateProperty.resolveWith(
            //       (states) => states.contains(MaterialState.hovered) ? 2 : 0,
            //     ),
            //     padding: MaterialStateProperty.all(
            //       EdgeInsets.symmetric(horizontal: 20, vertical: 16),
            //     ),
            //     overlayColor: MaterialStateProperty.resolveWith(
            //       (states) => states.contains(MaterialState.hovered)
            //           ? Colors.white12
            //           : Colors.white.withOpacity(0),
            //     ),
            //     foregroundColor: MaterialStateProperty.all(Colors.white),
            //   ),
            //   child: Text('Open SemDoc'),
            // ),
            Text(
              'To open a file,\ndrag it here or click anywhere.',
              textAlign: TextAlign.center,
            ),
            Spacer(),
            // TODO: Add recently opened documents.
            // TODO: Add suggestion to make this reader the default opener for `.semdoc` files.
            // TODO: Maybe highlight specific tools?
            Opacity(
              opacity: 0.5,
              child: Text(
                'By the way: You can press space to enter commands.',
                textAlign: TextAlign.center,
              ),
            ),
            SizedBox(height: 8),
          ],
        ),
      ),
    );
  }
}

class DocumentScreen extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: Colors.white,
      // appBar: AppBar(
      //   title: Text('SemDoc', style: TextStyle(color: Colors.black)),
      //   backgroundColor: Colors.white,
      //   foregroundColor: Colors.black,
      // ),
      body: LayoutBuilder(builder: (context, constraints) {
        if (constraints.maxWidth > 700) {
          return Row(
            children: [
              Container(
                width: 200,
                height: double.infinity,
                color: Colors.amber,
                child: Text('Table of contents'),
              ),
              Expanded(
                child: Center(child: SizedBox(width: 700, child: DummyDoc())),
              ),
            ],
          );
        } else {
          return DummyDoc();
        }
      }),
    );
  }
}

class DummyDoc extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return ListView(
      padding: EdgeInsets.all(16) + EdgeInsets.only(top: 16, bottom: 64),
      children: [
        Text(
          'SemDoc',
          style: GoogleFonts.josefinSans(
            fontWeight: FontWeight.w900,
            fontSize: 40,
            height: 1,
          ),
        ),
        SizedBox(height: 8),
        Text(
            'Hello, world! This is a test. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Donec vulputate sed risus at egestas. Suspendisse tempus varius purus, vel pellentesque lacus tristique vulputate. Pellentesque sem diam, finibus sed eros ut, scelerisque accumsan libero. Aenean at quam sit amet ipsum vulputate fringilla et sit amet nunc. Phasellus ultrices eu est eu blandit. Integer convallis felis sem, et condimentum orci lobortis a. Suspendisse vestibulum purus sed neque consectetur, quis consectetur nisl ullamcorper. Nunc orci mauris, venenatis ut ex id, blandit posuere magna. Curabitur efficitur, massa venenatis scelerisque sollicitudin, ipsum diam lacinia ex, faucibus ultrices ex ante consectetur elit. Nunc pharetra, eros vitae cursus sollicitudin, tellus justo convallis diam, cursus fermentum nibh quam ut mauris. Morbi laoreet odio libero, sit amet laoreet dolor facilisis mollis. Ut volutpat risus quis ex suscipit rhoncus. Nullam dapibus ac nisl eget rhoncus. Integer id libero ac purus accumsan malesuada non quis libero. Mauris sit amet dui congue, condimentum nisi sit amet, vulputate urna. Vivamus dictum pharetra ligula quis lobortis. Nullam nec tellus tellus. Etiam quis aliquam ex.'),
        SizedBox(height: 8),
        Text('This is a test. Hello!'),
        SizedBox(height: 8),
        Placeholder(),
        SizedBox(height: 8),
        Text('Lorem ipsum'),
        SizedBox(height: 8),
        Text(
            'Hello, world! This is a test. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Donec vulputate sed risus at egestas. Suspendisse tempus varius purus, vel pellentesque lacus tristique vulputate. Pellentesque sem diam, finibus sed eros ut, scelerisque accumsan libero. Aenean at quam sit amet ipsum vulputate fringilla et sit amet nunc. Phasellus ultrices eu est eu blandit. Integer convallis felis sem, et condimentum orci lobortis a. Suspendisse vestibulum purus sed neque consectetur, quis consectetur nisl ullamcorper. Nunc orci mauris, venenatis ut ex id, blandit posuere magna. Curabitur efficitur, massa venenatis scelerisque sollicitudin, ipsum diam lacinia ex, faucibus ultrices ex ante consectetur elit. Nunc pharetra, eros vitae cursus sollicitudin, tellus justo convallis diam, cursus fermentum nibh quam ut mauris. Morbi laoreet odio libero, sit amet laoreet dolor facilisis mollis. Ut volutpat risus quis ex suscipit rhoncus. Nullam dapibus ac nisl eget rhoncus. Integer id libero ac purus accumsan malesuada non quis libero. Mauris sit amet dui congue, condimentum nisi sit amet, vulputate urna. Vivamus dictum pharetra ligula quis lobortis. Nullam nec tellus tellus. Etiam quis aliquam ex.'),
      ],
    );
  }
}

class Placeholder extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return Material(
      shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(8)),
      color: Colors.black12,
      child: Container(height: 100),
    );
  }
}
