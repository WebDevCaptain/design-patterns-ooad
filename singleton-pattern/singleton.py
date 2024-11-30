class Singleton:
    _instance = None

    def __new__(cls, *args, **kwargs):
        if cls._instance is None:
            cls._instance = super(Singleton, cls).__new__(cls)

        return cls._instance


# Using singleton
s1 = Singleton()
s2 = Singleton()

print(f"Both instances are same {s1 is s2}")
