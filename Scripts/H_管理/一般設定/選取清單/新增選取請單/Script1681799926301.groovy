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

WebUI.setText(findTestObject('Object Repository/管理/一般設定/選取清單/新增選取清單/input_(15trunk)_member_id'), 'catc')

WebUI.setEncryptedText(findTestObject('Object Repository/管理/一般設定/選取清單/新增選取清單/input_(15trunk)_member_pwd'), 'eh9+mX6n5G+m9qEgDyu51A==')

WebUI.click(findTestObject('Object Repository/管理/一般設定/選取清單/新增選取清單/input_(15trunk)_Submit'))

WebUI.click(findTestObject('Object Repository/管理/一般設定/選取清單/新增選取清單/a_'))

WebUI.maximizeWindow()

WebUI.enableSmartWait()

WebUI.scrollToElement(findTestObject('管理/一般設定/選取清單/新增選取清單/a__1'), 2)

WebUI.enhancedClick(findTestObject('Object Repository/管理/一般設定/選取清單/新增選取清單/a__1'))

WebUI.verifyElementText(findTestObject('Object Repository/管理/一般設定/選取清單/新增選取清單/a__1'), '一般設定')

WebUI.verifyElementText(findTestObject('Object Repository/管理/一般設定/選取清單/新增選取清單/a__1_2'), '選取清單')

WebUI.click(findTestObject('Object Repository/管理/一般設定/選取清單/新增選取清單/a__1_2'))

WebUI.scrollToElement(findTestObject('管理/一般設定/選取清單/新增選取清單/input_(15trunk)_Submit'), 1)

WebUI.verifyElementPresent(findTestObject('Object Repository/管理/一般設定/選取清單/新增選取清單/input_(15trunk)_Submit'), 1)

WebUI.enhancedClick(findTestObject('Object Repository/管理/一般設定/選取清單/新增選取清單/input_(15trunk)_Submit'))

WebUI.verifyElementText(findTestObject('Object Repository/管理/一般設定/選取清單/新增選取清單/td_'), '值:')

WebUI.click(findTestObject('Object Repository/管理/一般設定/選取清單/新增選取清單/input__TextField'))

WebUI.setText(findTestObject('Object Repository/管理/一般設定/選取清單/新增選取清單/input__TextField'), 'ABC')

WebUI.setText(findTestObject('Object Repository/管理/一般設定/選取清單/新增選取清單/input__TextField_0'), '')

WebUI.click(findTestObject('Object Repository/管理/一般設定/選取清單/新增選取清單/input__TextField_0'))

WebUI.setText(findTestObject('Object Repository/管理/一般設定/選取清單/新增選取清單/input__TextField_0'), '0')

WebUI.setText(findTestObject('Object Repository/管理/一般設定/選取清單/新增選取清單/input__TextField_1'), '[NT]')

WebUI.click(findTestObject('Object Repository/管理/一般設定/選取清單/新增選取清單/input__TextField_1'))

WebUI.click(findTestObject('Object Repository/管理/一般設定/選取清單/新增選取清單/td__1'))

WebUI.setText(findTestObject('Object Repository/管理/一般設定/選取清單/新增選取清單/input__TextField_1'), '測試用')

WebUI.setText(findTestObject('Object Repository/管理/一般設定/選取清單/新增選取清單/input__TextField_2'), '1')

WebUI.selectOptionByValue(findTestObject('Object Repository/管理/一般設定/選取清單/新增選取清單/select_truefalse'), '', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/管理/一般設定/選取清單/新增選取清單/select_truefalse'), 'true', true)

WebUI.click(findTestObject('Object Repository/管理/一般設定/選取清單/新增選取清單/input__Submit_0'))

WebUI.click(findTestObject('Object Repository/管理/一般設定/選取清單/新增選取清單/div_'))

WebUI.click(findTestObject('Object Repository/管理/一般設定/選取清單/新增選取清單/div_'))

WebUI.verifyElementText(findTestObject('Object Repository/管理/一般設定/選取清單/新增選取清單/div_'), '存檔成功')

WebUI.verifyElementText(findTestObject('Object Repository/管理/一般設定/選取清單/新增選取清單/span_'), '確定')

WebUI.click(findTestObject('Object Repository/管理/一般設定/選取清單/新增選取清單/span_'))

WebUI.verifyElementText(findTestObject('Object Repository/管理/一般設定/選取清單/新增選取清單/a_ABC'), 'ABC')

WebUI.takeFullPageScreenshotAsCheckpoint('新增選取清單成功')

WebUI.click(findTestObject('Object Repository/管理/一般設定/選取清單/新增選取清單/a__1_2_3'))

WebUI.closeBrowser()

