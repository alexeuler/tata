import 'package:flutter/material.dart';
import 'package:tata_mobile/screens/onboarding.dart';

void main() {
  runApp(TataApp());
}

class TataApp extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Tata',
      theme: ThemeData.light(),
      home: Onboarding(),
    );
  }
}
