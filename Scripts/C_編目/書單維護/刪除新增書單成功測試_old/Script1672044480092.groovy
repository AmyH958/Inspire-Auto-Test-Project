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

WebUI.setText(findTestObject('Object Repository/Page_(15trunk)/input_(15trunk)_member_id'), 'catc')

WebUI.delay(2)

WebUI.setEncryptedText(findTestObject('Object Repository/Page_(15trunk)/input_(15trunk)_member_pwd'), 'eh9+mX6n5G+m9qEgDyu51A==')

WebUI.click(findTestObject('Object Repository/Page_(15trunk)/input_(15trunk)_Submit'))

WebUI.click(findTestObject('Object Repository/Page_(15trunk)/a_'))

WebUI.click(findTestObject('Object Repository/Page_(15trunk)/a__1'))

WebUI.mouseOver(findTestObject('Page_(15trunk)/td_()'))

WebUI.scrollToElement(findTestObject('Page_(15trunk)/td_()'), 2)

delete_book_name = WebUI.verifyElementText(findTestObject('Object Repository/Page_(15trunk)/td_()'), '白雪壞公主(測試)')

WebUI.maximizeWindow()

while (true) {
	
}

WebUI.scrollToElement(findTestObject('New Page(15trunk)/Page_(15trunk)/img__Any_0_8'), 2, FailureHandling.CONTINUE_ON_FAILURE)

WebUI.verifyElementPresent(findTestObject('New Page(15trunk)/Page_(15trunk)/img__Any_0_8'), 2, FailureHandling.CONTINUE_ON_FAILURE)

WebUI.delay(2)

WebUI.mouseOver(findTestObject('New Page(15trunk)/Page_(15trunk)/img__Any_0_8'), FailureHandling.CONTINUE_ON_FAILURE)

WebUI.delay(2, FailureHandling.CONTINUE_ON_FAILURE)

WebUI.delay(2, FailureHandling.CONTINUE_ON_FAILURE)

WebUI.scrollToElement(findTestObject('New Page(15trunk)/Page_(15trunk)/img__Any_0_8'), 2, FailureHandling.CONTINUE_ON_FAILURE)

WebUI.mouseOver(findTestObject('New Page(15trunk)/Page_(15trunk)/img__Any_0_8'), FailureHandling.CONTINUE_ON_FAILURE)

WebUI.waitForElementClickable(findTestObject('New Page(15trunk)/Page_(15trunk)/img__Any_0_8'), 2, FailureHandling.CONTINUE_ON_FAILURE)

WebUI.click(findTestObject('New Page(15trunk)/Page_(15trunk)/img__Any_0_8'), FailureHandling.CONTINUE_ON_FAILURE)

WebUI.delay(2, FailureHandling.CONTINUE_ON_FAILURE)

WebUI.click(findTestObject('Object Repository/Page_(15trunk)/div_Portal    ERM FRBRFRBRFRBR--()()-- QR C_b83f78'))

WebUI.takeFullPageScreenshotAsCheckpoint('刪除新增書單成功')

WebUI.closeBrowser()

