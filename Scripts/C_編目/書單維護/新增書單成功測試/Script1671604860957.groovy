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

WebUI.click(findTestObject('New Page(15trunk)/Page_(15trunk)/a_'))

WebUI.verifyElementText(findTestObject('New Page(15trunk)/Page_(15trunk)/a_'), '編目')

WebUI.click(findTestObject('New Page(15trunk)/Page_(15trunk)/a__1 - Copy'))

WebUI.maximizeWindow()

WebUI.click(findTestObject('New Page(15trunk)/Page_(15trunk)/a__1_2 - Copy'))

WebUI.setText(findTestObject('Object Repository/New Page(15trunk)/Page_(15trunk)/input__TextField_0'), '白雪壞公主(測試)')

WebUI.setText(findTestObject('Object Repository/New Page(15trunk)/Page_(15trunk)/textarea__TextArea'), '有夠壞的公主')

WebUI.setText(findTestObject('Object Repository/New Page(15trunk)/Page_(15trunk)/textarea_Text In Opac_TextArea_0'), 'aaa')

WebUI.click(findTestObject('Object Repository/New Page(15trunk)/Page_(15trunk)/input__elementName'))

WebUI.verifyElementPresent(findTestObject('New Page(15trunk)/Page_(15trunk)/img__Any_0_8', [('variable') : '白雪壞公主']), 0, 
    FailureHandling.CONTINUE_ON_FAILURE)

WebUI.click(findTestObject('Object Repository/New Page(15trunk)/Page_(15trunk)/img__Any_0_8'))

WebUI.click(findTestObject('Object Repository/New Page(15trunk)/Page_(15trunk)/img__jwindow.dlocations1'))

WebUI.click(findTestObject('Object Repository/New Page(15trunk)/Page_(15trunk)/a_e-resources -'))

WebUI.click(findTestObject('Object Repository/New Page(15trunk)/Page_(15trunk)/td_'))

WebUI.click(findTestObject('Object Repository/New Page(15trunk)/Page_(15trunk)/td_'))

WebUI.setText(findTestObject('Object Repository/New Page(15trunk)/Page_(15trunk)/input__dateBegin'), '2022/12/1')

WebUI.click(findTestObject('Object Repository/New Page(15trunk)/Page_(15trunk)/input__dateEnd'))

WebUI.setText(findTestObject('Object Repository/New Page(15trunk)/Page_(15trunk)/input__dateEnd'), '2023/12/31')

WebUI.click(findTestObject('Object Repository/New Page(15trunk)/Page_(15trunk)/td_'))

WebUI.verifyElementPresent(findTestObject('New Page(15trunk)/Page_(15trunk)/a__1_2_3 - Copy'), 0)

WebUI.verifyElementText(findTestObject('New Page(15trunk)/Page_(15trunk)/a__1_2_3 - Copy'), '修改/存檔')

WebUI.click(findTestObject('New Page(15trunk)/Page_(15trunk)/a__1_2_3 - Copy'))

WebUI.takeFullPageScreenshotAsCheckpoint('Add Book Success')

WebUI.verifyElementText(findTestObject('Object Repository/New Page(15trunk)/Page_(15trunk)/td_()'), '白雪壞公主(測試)', FailureHandling.CONTINUE_ON_FAILURE)

WebUI.click(findTestObject('Object Repository/New Page(15trunk)/Page_(15trunk)/a__1_2_3_4'))

WebUI.closeBrowser()

