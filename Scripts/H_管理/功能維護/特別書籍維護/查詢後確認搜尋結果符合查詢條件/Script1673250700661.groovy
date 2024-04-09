import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.openBrowser('')

WebUI.navigateToUrl('http://10.11.20.15/inspireapp/logon')

WebUI.delay(2)

WebUI.setText(findTestObject('Object Repository/New Page(15trunk)/Page_(15trunk)/input_(15trunk)_member_id'), 'catc')

WebUI.delay(2)

WebUI.setEncryptedText(findTestObject('Object Repository/New Page(15trunk)/Page_(15trunk)/input_(15trunk)_member_pwd'), 
    'eh9+mX6n5G+m9qEgDyu51A==')

WebUI.delay(2)

WebUI.click(findTestObject('Object Repository/New Page(15trunk)/Page_(15trunk)/input_(15trunk)_Submit'))

WebUI.maximizeWindow()

WebUI.delay(2)

WebUI.click(findTestObject('New Page(15trunk)/Page_(15trunk)/Page_(15trunk)/Clean button'))

WebUI.delay(2)

WebUI.click(findTestObject('Object Repository/New Page(15trunk)/Page_(15trunk)/a_maintain'))

WebUI.verifyElementText(findTestObject('Object Repository/New Page(15trunk)/Page_(15trunk)/a_maintain'), '管理')

WebUI.verifyElementText(findTestObject('Object Repository/New Page(15trunk)/Page_(15trunk)/a__1'), '功能維護')

WebUI.click(findTestObject('Object Repository/New Page(15trunk)/Page_(15trunk)/a__1'))

WebUI.rightClick(findTestObject('Object Repository/New Page(15trunk)/Page_(15trunk)/a__1_2'))

WebUI.verifyElementText(findTestObject('Object Repository/New Page(15trunk)/Page_(15trunk)/a__1_2'), '特別書籍維護')

WebUI.click(findTestObject('Object Repository/New Page(15trunk)/Page_(15trunk)/a__1_2'))

WebUI.setText(findTestObject('Object Repository/New Page(15trunk)/Page_(15trunk)/input__field1'), '致勝')

WebUI.setText(findTestObject('Object Repository/New Page(15trunk)/Page_(15trunk)/input__field4'), '777001')

WebUI.verifyElementPresent(findTestObject('Object Repository/New Page(15trunk)/Page_(15trunk)/input__formSubmitSearch'), 
    0)

WebUI.click(findTestObject('Object Repository/New Page(15trunk)/Page_(15trunk)/input__formSubmitSearch'))

WebUI.verifyElementText(findTestObject('New Page(15trunk)/Page_(15trunk)/a_777001'), '777001')

致勝 = WebUI.verifyElementText(findTestObject('New Page(15trunk)/Page_(15trunk)/td_a(Colin Poewell), .(Tony Koltz)     ,2014103', 
        [('variable') : '致勝']), '致勝領導 :鮑爾的人生體悟 /柯林.鮑爾a(Colin Poewell), 東尼.寇茲(Tony Koltz)編著 ; 黃國賢翻譯 ; 國防部譯印台北市 :國防部政務辦公室,2014[民103]')

WebUI.takeFullPageScreenshotAsCheckpoint('Special book result')

WebUI.click(findTestObject('Object Repository/New Page(15trunk)/Page_(15trunk)/a__1_2_3'))

WebUI.closeBrowser()

