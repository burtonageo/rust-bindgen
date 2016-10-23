@interface EmptyClass
@end

@interface ClassWithMethods: EmptyClass
-(void)foo;
-(int)bar:(int)param;
-(long)doThing:(long)thing withOtherThing:(void*)otherThing;
@end

@interface InheritanceClass: ClassWithMethods
-(void*)otherMethod;
@end
