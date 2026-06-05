class ParseResult:
    """
    A class represent parse result.

    パース処理の結果を表すクラスです。
    """

    address: dict[str, str]
    """
    都道府県名、市区町村名、町名、それ以降の文字列をそれぞれ格納する辞書型を返します。
    
    {prefecture: str, city: str, town: str, rest: str}
    """

    error: dict[str, str]
    """
    パース処理中にエラーが発生した場合、エラーのタイプとエラーメッセージを格納する辞書型を返します。
    
    {error_type: str, error_message: str}
    """


def parse(address: str) -> ParseResult:
    """
    Format informal address into formal style

    入力された住所を正式な表記に整形します。

    :param address: 住所
    :return: ParseResult
    """


async def parse_async(address: str) -> ParseResult:
    """
    Format informal address into formal style (async)

    入力された住所を正式な表記に整形します（非同期版）。

    :param address: 住所
    :return: ParseResult
    """


class Parser:
    def __new__(cls) -> Parser:
        """
        Construct a parser.
    
        パーサーを生成します。

        :return: JapaneseAddressParser
        """

    def parse(self, address: str) -> ParseResult:
        """
        Format informal address into formal style

        入力された住所を正式な表記に整形します。

        :param address: 住所
        :return: ParseResult
        """
