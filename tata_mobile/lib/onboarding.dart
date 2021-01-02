import 'package:flutter/material.dart';

class Onboarding extends StatefulWidget {
  Onboarding({Key key}) : super(key: key);

  _OnboardingState createState() => _OnboardingState();
}

class _OnboardingState extends State<Onboarding> {
  PageController _pageController;

  @override
  void initState() {
    super.initState();
    _pageController = PageController();
  }

  @override
  void dispose() {
    _pageController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return PageView(controller: _pageController, children: [
      _welcomePage,
      _pickName,
    ]);
  }
}

final _welcomePage = Container(
    color: Colors.pink,
    child: Column(
      children: [Text("Welcome")],
    ));

final _pickName = Container(
  color: Colors.blue,
  child: Column(
    children: [Text("Pick Username")],
  ),
);
